//! One consented Chrome connection, leased to one CLI command at a time.
//!
//! The helper has no TCP listener. Its Unix sockets live in a freshly created
//! 0700 directory, and disappear when the upstream connection ends. It never
//! rediscovers, reconnects, launches Chrome, or writes browser content to disk.

use crate::error::Error;

#[cfg(unix)]
mod relay;

pub fn disconnected() -> Error {
    Error::BrowserDisconnected(
        "the approved session has ended; run lsearch connect --existing and approve again. No automatic reconnection was attempted".to_owned(),
    )
}

#[cfg(unix)]
pub use unix::{PendingSession, serve, start, stop};

#[cfg(not(unix))]
pub fn unsupported() -> Error {
    Error::Unsupported {
        backend: "this platform".to_owned(),
        feature: "persistent existing-Chrome sessions (macOS/Linux required)".to_owned(),
    }
}

#[cfg(not(unix))]
pub struct PendingSession {
    pub session: crate::config::BrowserSession,
}
#[cfg(not(unix))]
impl PendingSession {
    pub async fn commit(self) -> crate::error::Result<()> {
        Err(unsupported())
    }
}
#[cfg(not(unix))]
pub async fn start(_: &str, _: u64) -> crate::error::Result<PendingSession> {
    Err(unsupported())
}
#[cfg(not(unix))]
pub async fn stop(_: &crate::config::BrowserSession) -> crate::error::Result<bool> {
    Err(unsupported())
}
#[cfg(not(unix))]
pub async fn serve() -> crate::error::Result<()> {
    Err(unsupported())
}

#[cfg(unix)]
mod unix {
    use crate::{
        browser::cdp::CdpClient,
        config::BrowserSession,
        error::{Error, IoContext, Result},
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::{
        fs::{DirBuilder, Permissions},
        os::unix::fs::{DirBuilderExt, PermissionsExt},
        path::PathBuf,
        process::Stdio,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };
    use tokio::{
        io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::{UnixListener, UnixStream},
        process::{Child, ChildStdin, Command},
    };
    use tokio_tungstenite::tungstenite::Error as WsError;

    #[derive(Serialize, Deserialize)]
    struct Boot {
        session: BrowserSession,
        timeout_ms: u64,
    }

    struct Directory(PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            // Never recursively delete a directory supplied in saved config.
            let _ = std::fs::remove_file(self.0.join("cdp"));
            let _ = std::fs::remove_file(self.0.join("control"));
            let _ = std::fs::remove_dir(&self.0);
        }
    }

    pub struct PendingSession {
        pub session: BrowserSession,
        child: Child,
        input: ChildStdin,
        committed: bool,
    }

    impl PendingSession {
        pub async fn commit(mut self) -> Result<()> {
            self.input
                .write_all(b"commit\n")
                .await
                .at("browser session helper")?;
            self.committed = true;
            Ok(())
        }
    }

    impl Drop for PendingSession {
        fn drop(&mut self) {
            if !self.committed {
                let _ = self.child.start_kill();
                drop(Directory(self.session.directory.clone()));
            }
        }
    }

    pub async fn start(endpoint: &str, timeout_ms: u64) -> Result<PendingSession> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!("ls-{}-{nonce:x}", std::process::id()));
        DirBuilder::new()
            .mode(0o700)
            .create(&directory)
            .at("private browser session directory")?;
        let session = BrowserSession {
            directory,
            endpoint: endpoint.to_owned(),
        };
        let mut child = match Command::new(std::env::current_exe().at("lsearch executable")?)
            .arg("__browser-session")
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                drop(Directory(session.directory));
                return Err(Error::Io {
                    path: "browser session helper".to_owned(),
                    source: error,
                });
            }
        };
        let input = child.stdin.take().expect("piped stdin");
        let output = child.stdout.take().expect("piped stdout");
        let mut pending = PendingSession {
            session,
            child,
            input,
            committed: false,
        };
        let boot = Boot {
            session: pending.session.clone(),
            timeout_ms,
        };
        pending
            .input
            .write_all(format!("{}\n", serde_json::to_string(&boot)?).as_bytes())
            .await
            .at("browser session helper")?;
        let mut line = String::new();
        tokio::time::timeout(
            Duration::from_millis(timeout_ms.saturating_mul(2).saturating_add(5_000)),
            BufReader::new(output).read_line(&mut line),
        )
        .await
        .map_err(|_| Error::BrowserApprovalTimeout { timeout_ms })?
        .at("browser session helper")?;
        let result: Value = serde_json::from_str(&line).map_err(|_| {
            Error::BrowserConnectionFailed("local helper exited before it was ready".to_owned())
        })?;
        match result["error"]["code"].as_str() {
            Some("browser_approval_timeout") => {
                return Err(Error::BrowserApprovalTimeout { timeout_ms });
            }
            Some("browser_approval_denied") => return Err(Error::BrowserApprovalDenied),
            Some("browser_disconnected") => return Err(super::disconnected()),
            Some(_) => {
                return Err(Error::BrowserConnectionFailed(
                    result["error"]["message"]
                        .as_str()
                        .unwrap_or("local helper failed")
                        .to_owned(),
                ));
            }
            None if result["ok"] == true => {}
            None => {
                return Err(Error::BrowserConnectionFailed(
                    "invalid local helper response".to_owned(),
                ));
            }
        }
        Ok(pending)
    }

    pub async fn stop(session: &BrowserSession) -> Result<bool> {
        let path = session.directory.join("control");
        let mut stream = match UnixStream::connect(&path).await {
            Ok(stream) => stream,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused
                ) =>
            {
                return Ok(false);
            }
            Err(error) => {
                return Err(Error::Io {
                    path: path.display().to_string(),
                    source: error,
                });
            }
        };
        tokio::time::timeout(Duration::from_secs(3), async {
            stream.write_all(b"stop\n").await?;
            let ack = stream.read_u8().await?;
            if ack != 1 {
                return Err(std::io::Error::other("invalid disconnect acknowledgement"));
            }
            Ok(())
        })
        .await
        .map_err(|_| Error::Timeout {
            operation: "disconnect local helper".to_owned(),
            timeout_ms: 3_000,
        })?
        .at("browser session helper")?;
        Ok(true)
    }

    fn approval_error(error: Error, timeout_ms: u64) -> Error {
        match error {
            Error::Timeout { .. } => Error::BrowserApprovalTimeout { timeout_ms },
            Error::WebSocket(WsError::Http(response))
                if matches!(response.status().as_u16(), 401 | 403) =>
            {
                Error::BrowserApprovalDenied
            }
            Error::WebSocket(WsError::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::NotFound
                ) =>
            {
                super::disconnected()
            }
            other => Error::BrowserConnectionFailed(other.to_string()),
        }
    }

    pub async fn serve() -> Result<()> {
        let mut input = BufReader::new(tokio::io::stdin());
        let mut line = String::new();
        input.read_line(&mut line).await.at("helper startup")?;
        let boot: Boot = serde_json::from_str(&line)?;
        let _directory = Directory(boot.session.directory.clone());
        let setup = async {
            let listener = UnixListener::bind(boot.session.directory.join("cdp"))
                .at("private command socket")?;
            let control = UnixListener::bind(boot.session.directory.join("control"))
                .at("private control socket")?;
            for name in ["cdp", "control"] {
                std::fs::set_permissions(
                    boot.session.directory.join(name),
                    Permissions::from_mode(0o600),
                )
                .at("private socket permissions")?;
            }
            let mut client = CdpClient::connect(&boot.session.endpoint, boot.timeout_ms)
                .await
                .map_err(|error| approval_error(error, boot.timeout_ms))?;
            client
                .verify()
                .await
                .map_err(|error| approval_error(error, boot.timeout_ms))?;
            Ok::<_, Error>((client.into_browser_socket(), listener, control))
        }
        .await;
        let (socket, listener, control) = match setup {
            Ok(parts) => parts,
            Err(error) => {
                println!("{}", crate::output::render_error(&error, false));
                return Ok(());
            }
        };
        println!("{{\"ok\":true}}");
        // EOF or an abandoned parent closes the approved connection, not an
        // orphan helper. Only the parent that saved the selection commits it.
        line.clear();
        if tokio::time::timeout(Duration::from_secs(5), input.read_line(&mut line))
            .await
            .ok()
            .and_then(std::result::Result::ok)
            .is_none()
            || line != "commit\n"
        {
            return Ok(());
        }
        super::relay::run(socket, listener, control).await
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn stalled_chrome_handshake_is_an_approval_timeout_not_a_disconnect() {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("ws://{}", listener.local_addr().unwrap());
            let error = CdpClient::connect(&endpoint, 50).await.err().unwrap();
            assert_eq!(approval_error(error, 50).code(), "browser_approval_timeout");
        }

        #[test]
        fn rejected_approval_and_closed_port_have_distinct_codes() {
            let denied = WsError::Http(Box::new(
                http::Response::builder().status(403).body(None).unwrap(),
            ));
            assert_eq!(
                approval_error(Error::WebSocket(denied), 100).code(),
                "browser_approval_denied"
            );
            let refused = WsError::Io(std::io::Error::from(std::io::ErrorKind::ConnectionRefused));
            assert_eq!(
                approval_error(Error::WebSocket(refused), 100).code(),
                "browser_disconnected"
            );
            let unknown = WsError::Io(std::io::Error::from(std::io::ErrorKind::ConnectionReset));
            assert_eq!(
                approval_error(Error::WebSocket(unknown), 100).code(),
                "browser_connection_failed"
            );
        }
    }
}
