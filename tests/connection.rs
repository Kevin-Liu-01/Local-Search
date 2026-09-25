use std::{
    net::TcpListener,
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

use assert_cmd::Command;
use futures_util::{SinkExt, StreamExt};
use predicates::prelude::*;
use serde_json::{Value, json};
use tempfile::TempDir;
use tokio_tungstenite::tungstenite::{
    Message,
    handshake::server::{Request, Response},
};

struct Browser {
    endpoint: String,
    port: u16,
    calls: Arc<Mutex<Vec<Value>>>,
    stop: Arc<AtomicBool>,
    worker: Option<thread::JoinHandle<()>>,
    connections: Arc<AtomicUsize>,
}

impl Browser {
    // The handshake callback's response type is prescribed by tungstenite.
    #[allow(clippy::result_large_err)]
    fn start(approve: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        listener.set_nonblocking(true).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let calls = Arc::new(Mutex::new(Vec::new()));
        let worker_stop = stop.clone();
        let worker_calls = calls.clone();
        let connections = Arc::new(AtomicUsize::new(0));
        let worker_connections = connections.clone();
        let worker = thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
                let listener = tokio::net::TcpListener::from_std(listener).unwrap();
                let mut has_agent_tab = false;
                let mut deferred: Option<Value> = None;
                while !worker_stop.load(Ordering::Relaxed) {
                    let Ok(Ok((socket, _))) = tokio::time::timeout(Duration::from_millis(50), listener.accept()).await else { continue };
                    worker_connections.fetch_add(1, Ordering::SeqCst);
                    if !approve { drop(socket); continue; }
                    let Ok(mut socket) = tokio_tungstenite::accept_hdr_async(socket, |request: &Request, response: Response| {
                        assert!(!request.headers().contains_key("Origin"));
                        Ok(response)
                    }).await else { continue };
                    while !worker_stop.load(Ordering::Relaxed) {
                        let next = tokio::time::timeout(Duration::from_millis(50), socket.next()).await;
                        let message = match next { Err(_) => continue, Ok(Some(Ok(Message::Text(text)))) => text, _ => break };
                        let call: Value = serde_json::from_str(&message).unwrap();
                        worker_calls.lock().unwrap().push(call.clone());
                        if call["method"] == "Fixture.delay" { deferred = Some(call); continue; }
                        if let Some(previous) = deferred.take() {
                            let late = json!({"id":previous["id"],"result":{"stale":true}});
                            if socket.send(Message::Text(late.to_string().into())).await.is_err() { break; }
                        }
                        let result = match call["method"].as_str().unwrap() {
                            "Browser.getVersion" => json!({"product":"Chrome/144.fixture"}),
                            "Target.getTargets" => {
                                let mut targets = vec![json!({"targetId":"personal-tab","type":"page","title":"Fixture","url":"https://example.com/"})];
                                if has_agent_tab { targets.push(json!({"targetId":"agent-tab","type":"page","title":"Agent","url":"about:blank"})); }
                                json!({"targetInfos":targets})
                            }
                            "Target.createTarget" => { has_agent_tab = true; json!({"targetId":"agent-tab"}) }
                            "Target.attachToTarget" => json!({"sessionId":"fixture-session"}),
                            "Runtime.evaluate" => json!({"result":{"value":{"fixture":true}}}),
                            _ => json!({}),
                        };
                        let response = json!({"id":call["id"],"result":result});
                        if socket.send(Message::Text(response.to_string().into())).await.is_err() { break; }
                    }
                }
            });
        });
        Self {
            endpoint: format!("ws://127.0.0.1:{port}/devtools/browser/fixture"),
            port,
            calls,
            stop,
            worker: Some(worker),
            connections,
        }
    }

    fn write_profile(&self, profile: &Path) {
        std::fs::write(
            profile.join("DevToolsActivePort"),
            format!("{}\n/devtools/browser/fixture\n", self.port),
        )
        .unwrap();
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.worker.take().unwrap().join().unwrap();
    }
}

fn cli(config: &Path) -> Command {
    let mut cmd = Command::cargo_bin("lsearch").unwrap();
    cmd.env("LOCAL_SEARCH_CONFIG_DIR", config)
        .env("LOCAL_SEARCH_CACHE_DIR", config.join("cache"))
        .env("LOCAL_SEARCH_PLAIN", "1")
        .env_remove("LOCAL_SEARCH_CDP")
        .args(["--timeout", "500"]);
    cmd
}

fn saved(config: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(config.join("config.json")).unwrap()).unwrap()
}

#[test]
fn first_use_requires_a_choice_and_never_launches() {
    let dir = TempDir::new().unwrap();
    cli(dir.path())
        .args(["search", "fixture"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_not_configured"));
    assert!(!dir.path().join("chrome-profile").exists());
}

#[test]
#[cfg(unix)]
fn existing_browser_reuses_approval_but_restart_requires_explicit_reconnect() {
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let first = Browser::start(true);
    first.write_profile(profile.path());
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    assert_eq!(saved(dir.path())["connection"]["mode"], "existing");
    assert_eq!(
        first.calls.lock().unwrap()[0]["method"],
        "Browser.getVersion"
    );
    for _ in 0..3 {
        cli(dir.path()).args(["tabs", "list"]).assert().success();
    }
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    assert_eq!(first.connections.load(Ordering::SeqCst), 1);
    drop(first);
    let restarted = Browser::start(true);
    restarted.write_profile(profile.path());
    cli(dir.path())
        .args(["tabs", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    assert_eq!(restarted.connections.load(Ordering::SeqCst), 0);
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    cli(dir.path()).args(["tabs", "list"]).assert().success();
    assert_eq!(restarted.connections.load(Ordering::SeqCst), 1);
    cli(dir.path()).arg("disconnect").assert().success();
}

#[test]
fn denied_connection_does_not_replace_the_previous_choice() {
    let dir = TempDir::new().unwrap();
    let first = Browser::start(true);
    cli(dir.path())
        .args(["connect", &first.endpoint])
        .assert()
        .success();
    let previous = saved(dir.path());
    let denied = Browser::start(false);
    cli(dir.path())
        .args(["connect", &denied.endpoint])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    assert_eq!(saved(dir.path()), previous);
    cli(dir.path()).args(["tabs", "list"]).assert().success();
}

#[test]
#[cfg(unix)]
fn missing_existing_profile_never_falls_back_or_creates_a_profile() {
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let browser = Browser::start(true);
    browser.write_profile(profile.path());
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    let previous = saved(dir.path());
    std::fs::remove_file(profile.path().join("DevToolsActivePort")).unwrap();
    let calls = browser.calls.clone();
    drop(browser);
    cli(dir.path())
        .args(["search", "fixture"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    assert_eq!(calls.lock().unwrap().len(), 1);
    assert_eq!(saved(dir.path()), previous);
    assert!(!dir.path().join("chrome-profile").exists());
}

#[test]
fn explicit_override_does_not_change_selection_or_use_saved_target() {
    let dir = TempDir::new().unwrap();
    let first = Browser::start(true);
    cli(dir.path())
        .args(["connect", &first.endpoint, "--target", "personal-tab"])
        .assert()
        .success();
    let previous = saved(dir.path());
    let other = Browser::start(true);
    cli(dir.path())
        .args(["--cdp", &other.endpoint, "eval", "1"])
        .assert()
        .success();
    assert_eq!(saved(dir.path()), previous);
    let calls = other.calls.lock().unwrap();
    assert!(calls.iter().any(
        |call| call["method"] == "Target.createTarget" && call["params"]["background"] == true
    ));
    assert!(
        calls
            .iter()
            .all(|call| call["method"] != "Target.attachToTarget"
                || call["params"]["targetId"] != "personal-tab")
    );
}

#[test]
fn commands_reuse_their_background_tab_without_navigating_personal_tabs() {
    let dir = TempDir::new().unwrap();
    let browser = Browser::start(true);
    cli(dir.path())
        .args(["connect", &browser.endpoint])
        .assert()
        .success();
    for _ in 0..2 {
        cli(dir.path()).args(["eval", "1"]).assert().success();
    }
    let calls = browser.calls.lock().unwrap();
    assert_eq!(
        calls
            .iter()
            .filter(|call| call["method"] == "Target.createTarget")
            .count(),
        1
    );
    assert!(
        calls
            .iter()
            .all(|call| call["method"] != "Target.attachToTarget"
                || call["params"]["targetId"] == "agent-tab")
    );
    assert_eq!(saved(dir.path())["target_id"], "agent-tab");
    assert_eq!(saved(dir.path())["connection"]["mode"], "endpoint");
}

#[test]
fn legacy_saved_endpoint_stays_selected_and_fails_closed() {
    let dir = TempDir::new().unwrap();
    let browser = Browser::start(true);
    let previous = json!({"endpoint":browser.endpoint,"target_id":null});
    std::fs::write(dir.path().join("config.json"), previous.to_string()).unwrap();
    cli(dir.path()).args(["tabs", "list"]).assert().success();
    drop(browser);
    cli(dir.path())
        .args(["tabs", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    assert_eq!(saved(dir.path()), previous);
}

#[test]
fn corrupt_config_is_not_overwritten() {
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("config.json"), "{").unwrap();
    cli(dir.path())
        .args(["connect", "--managed"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("json_error"));
    assert_eq!(
        std::fs::read_to_string(dir.path().join("config.json")).unwrap(),
        "{"
    );
    assert!(!dir.path().join("chrome-profile").exists());
}

#[test]
fn conflicting_choices_are_rejected_before_connecting() {
    let dir = TempDir::new().unwrap();
    for args in [
        vec!["connect", "--existing", "--managed"],
        vec!["connect", "--existing", "9222"],
        vec!["connect", "--profile", "/tmp/fixture"],
        vec!["connect", "--existing", "--cdp", "9222"],
    ] {
        cli(dir.path()).args(args).assert().failure();
    }
    assert!(!dir.path().join("config.json").exists());
}

#[test]
fn malformed_devtools_endpoints_are_rejected() {
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    for raw in [
        "0\n/devtools/browser/test",
        "65536\n/devtools/browser/test",
        "9222\n//evil.example/browser",
        "9222\n/devtools/browser/test?token=bad",
    ] {
        std::fs::write(profile.path().join("DevToolsActivePort"), raw).unwrap();
        cli(dir.path())
            .args(["connect", "--existing", "--profile"])
            .arg(profile.path())
            .assert()
            .failure();
    }
    assert!(!dir.path().join("config.json").exists());
}

#[test]
fn cleanup_preserves_managed_choice_and_disconnection_is_reported() {
    let dir = TempDir::new().unwrap();
    let unused = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = unused.local_addr().unwrap().port();
    drop(unused);
    let previous = json!({
        "endpoint":format!("ws://127.0.0.1:{port}/devtools/browser/closed"),
        "target_id":"old-tab",
        "connection":{"mode":"managed","profile":dir.path().join("chrome-profile"),"port":port,"browser_path":null}
    });
    std::fs::write(dir.path().join("config.json"), previous.to_string()).unwrap();
    cli(dir.path())
        .args(["cleanup", "--kill", "--port", &port.to_string()])
        .assert()
        .success();
    let current = saved(dir.path());
    assert_eq!(current["connection"], previous["connection"]);
    assert!(current["endpoint"].is_null());
    cli(dir.path())
        .args(["tabs", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
}

#[test]
fn cleanup_refuses_to_kill_an_unrelated_listener() {
    let dir = TempDir::new().unwrap();
    let browser = Browser::start(true);
    cli(dir.path())
        .args(["connect", &browser.endpoint])
        .assert()
        .success();
    let previous = saved(dir.path());
    cli(dir.path())
        .args(["cleanup", "--kill", "--port", &browser.port.to_string()])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "only stops verified managed browsers",
        ));
    assert_eq!(saved(dir.path()), previous);
    cli(dir.path()).args(["tabs", "list"]).assert().success();
}

#[test]
fn cached_results_cannot_hide_a_disconnected_browser() {
    let dir = TempDir::new().unwrap();
    let browser = Browser::start(true);
    cli(dir.path())
        .args(["connect", &browser.endpoint])
        .assert()
        .success();
    let scope = browser.endpoint.clone();
    drop(browser);
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in scope
        .bytes()
        .chain([0])
        .chain("google".bytes())
        .chain([0])
        .chain("fixture".bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let cache = dir.path().join("cache");
    std::fs::create_dir(&cache).unwrap();
    std::fs::write(cache.join(format!("{hash:016x}.json")), json!({
        "scope":scope,"engine":"google","query":"fixture",
        "search":{"results":[{"rank":1,"title":"Fixture","url":"https://example.com/","domain":"example.com","snippet":"Fixture"}]}
    }).to_string()).unwrap();
    cli(dir.path())
        .args(["search", "fixture", "--limit", "1"])
        .assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("browser_disconnected"));
}

#[cfg(unix)]
#[test]
fn approved_session_is_private_serializes_commands_and_disconnect_keeps_chrome_open() {
    use std::os::unix::fs::PermissionsExt;
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let browser = Browser::start(true);
    browser.write_profile(profile.path());
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    let state = saved(dir.path());
    let socket_dir = std::path::PathBuf::from(state["session"]["directory"].as_str().unwrap());
    assert_eq!(
        std::fs::metadata(&socket_dir).unwrap().permissions().mode() & 0o777,
        0o700
    );
    for socket in ["cdp", "control"] {
        assert_eq!(
            std::fs::metadata(socket_dir.join(socket))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    thread::scope(|scope| {
        for _ in 0..4 {
            let config = dir.path();
            scope.spawn(move || {
                cli(config).args(["eval", "1"]).assert().success();
            });
        }
    });
    assert_eq!(browser.connections.load(Ordering::SeqCst), 1);
    let calls = browser.calls.lock().unwrap();
    assert_eq!(
        calls
            .iter()
            .filter(|call| call["method"] == "Target.createTarget")
            .count(),
        1
    );
    let mut ids = std::collections::HashSet::new();
    assert!(
        calls
            .iter()
            .all(|call| ids.insert(call["id"].as_u64().unwrap()))
    );
    drop(calls);
    cli(dir.path())
        .arg("disconnect")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"browserClosed\":false"));
    cli(dir.path())
        .args(["tabs", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    cli(dir.path()).arg("disconnect").assert().success();
    assert_eq!(browser.connections.load(Ordering::SeqCst), 1);
    assert!(
        browser
            .calls
            .lock()
            .unwrap()
            .iter()
            .all(|call| call["method"] != "Browser.close")
    );
    // The browser is still listening; only another explicit connect reauthorizes.
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    assert_eq!(browser.connections.load(Ordering::SeqCst), 2);
    cli(dir.path()).arg("disconnect").assert().success();
}

#[cfg(unix)]
#[test]
fn old_existing_selection_never_silently_creates_a_new_connection() {
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let browser = Browser::start(true);
    browser.write_profile(profile.path());
    let old = json!({"endpoint":browser.endpoint,"target_id":null,"connection":{"mode":"existing","profile":profile.path()}});
    std::fs::write(dir.path().join("config.json"), old.to_string()).unwrap();
    cli(dir.path())
        .args(["tabs", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_disconnected"));
    assert_eq!(browser.connections.load(Ordering::SeqCst), 0);
    assert_eq!(saved(dir.path()), old);
}

#[cfg(unix)]
#[test]
fn abandoned_setup_closes_approval_instead_of_leaving_an_orphan_helper() {
    let dir = TempDir::new().unwrap();
    let socket_dir = dir.path().join("session");
    std::fs::create_dir(&socket_dir).unwrap();
    let browser = Browser::start(true);
    let boot =
        json!({"session":{"directory":socket_dir,"endpoint":browser.endpoint},"timeout_ms":500});
    cli(dir.path())
        .arg("__browser-session")
        .write_stdin(format!("{boot}\n"))
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\":true"));
    assert!(!socket_dir.exists());
    // EOF instead of the parent's commit released the sole upstream socket.
    cli(dir.path())
        .args(["connect", &browser.endpoint])
        .assert()
        .success();
    assert_eq!(browser.connections.load(Ordering::SeqCst), 2);
}

#[cfg(unix)]
#[test]
fn approval_rejection_keeps_previous_choice_and_uses_its_own_error_code() {
    use std::io::{Read, Write};
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let previous = Browser::start(true);
    cli(dir.path())
        .args(["connect", &previous.endpoint])
        .assert()
        .success();
    let state = saved(dir.path());
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    std::fs::write(
        profile.path().join("DevToolsActivePort"),
        format!(
            "{}\n/devtools/browser/rejected\n",
            listener.local_addr().unwrap().port()
        ),
    )
    .unwrap();
    let worker = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut request = [0; 4096];
        assert!(stream.read(&mut request).unwrap() > 0);
        stream
            .write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
    });
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_approval_denied"));
    worker.join().unwrap();
    assert_eq!(saved(dir.path()), state);
    cli(dir.path()).args(["tabs", "list"]).assert().success();
}

#[test]
fn concurrent_selection_changes_fail_without_overwriting_state() {
    let dir = TempDir::new().unwrap();
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(dir.path().join("selection.lock"))
        .unwrap();
    lock.lock().unwrap();
    cli(dir.path())
        .arg("disconnect")
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_busy"));
    assert!(!dir.path().join("config.json").exists());
}

#[cfg(unix)]
#[test]
fn late_replies_never_cross_commands_and_disconnect_interrupts_an_active_lease() {
    let dir = TempDir::new().unwrap();
    let profile = TempDir::new().unwrap();
    let browser = Browser::start(true);
    browser.write_profile(profile.path());
    cli(dir.path())
        .args(["connect", "--existing", "--profile"])
        .arg(profile.path())
        .assert()
        .success();
    let state = saved(dir.path());
    let socket_path =
        std::path::Path::new(state["session"]["directory"].as_str().unwrap()).join("cdp");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let raw = tokio::net::UnixStream::connect(&socket_path).await.unwrap();
            let (mut first, _) = tokio_tungstenite::client_async("ws://localhost/session", raw)
                .await
                .unwrap();
            first
                .send(Message::Text(
                    json!({"id":1,"method":"Fixture.delay","params":{}})
                        .to_string()
                        .into(),
                ))
                .await
                .unwrap();
            first.close(None).await.unwrap();
            drop(first);
            let raw = tokio::net::UnixStream::connect(&socket_path).await.unwrap();
            let (mut second, _) = tokio_tungstenite::client_async("ws://localhost/session", raw)
                .await
                .unwrap();
            second
                .send(Message::Text(
                    json!({"id":1,"method":"Browser.getVersion","params":{}})
                        .to_string()
                        .into(),
                ))
                .await
                .unwrap();
            let response = tokio::time::timeout(Duration::from_secs(2), second.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap();
            let value: Value = serde_json::from_str(response.to_text().unwrap()).unwrap();
            assert_eq!(value["result"]["product"], "Chrome/144.fixture");
            assert!(value["result"].get("stale").is_none());
            second
                .send(Message::Text(
                    json!({"id":2,"method":"Fixture.delay","params":{}})
                        .to_string()
                        .into(),
                ))
                .await
                .unwrap();
            cli(dir.path()).arg("disconnect").assert().success();
            let closed = tokio::time::timeout(Duration::from_secs(2), second.next())
                .await
                .unwrap();
            assert!(matches!(
                closed,
                None | Some(Err(_) | Ok(Message::Close(_)))
            ));
        });
    assert_eq!(browser.connections.load(Ordering::SeqCst), 1);
    assert!(
        browser
            .calls
            .lock()
            .unwrap()
            .iter()
            .all(|call| call["method"] != "Browser.close")
    );
}
