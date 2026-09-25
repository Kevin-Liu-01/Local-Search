use std::path::{Path, PathBuf};

use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use url::Url;

use crate::{
    cli::BrowserKind,
    config,
    error::{Error, Result},
};

#[derive(Debug, Clone, Serialize)]
pub struct BrowserEndpoint {
    pub backend: String,
    pub websocket_url: String,
    pub source: String,
}

pub async fn discover(kind: BrowserKind, explicit: Option<&str>) -> Result<BrowserEndpoint> {
    if let Some(value) = explicit {
        return resolve_explicit(value).await;
    }

    if matches!(kind, BrowserKind::Safari) {
        return Err(Error::Unsupported {
            backend: "safari".to_owned(),
            feature: "normal-profile remote browser control".to_owned(),
        });
    }

    if let Ok(endpoint) = discover_chromium().await {
        return Ok(endpoint);
    }

    let stored = config::load().await.ok().and_then(|cfg| cfg.endpoint);
    if let Some(endpoint) = stored
        && let Ok(found) = resolve_explicit(&endpoint).await
    {
        return Ok(BrowserEndpoint {
            source: "config".to_owned(),
            ..found
        });
    }

    Err(Error::BrowserNotFound)
}

pub async fn doctor() -> serde_json::Value {
    let selection = match config::load().await {
        Ok(saved) => serde_json::json!({
            "connection": saved.connection,
            "endpoint": saved.endpoint,
            "note": "inspection only; doctor does not request Chrome approval or verify this connection"
        }),
        Err(error) => {
            serde_json::json!({"error": {"code": error.code(), "message": error.to_string()}})
        }
    };
    let mut endpoints = Vec::new();
    if let Ok(path) = config::managed_devtools_file()
        && let Ok(endpoint) = managed_endpoint_from_devtools_file(&path).await
    {
        endpoints.push(endpoint);
    }
    for path in devtools_files() {
        if let Ok(endpoint) = endpoint_from_devtools_file(&path).await {
            endpoints.push(endpoint);
        }
    }

    serde_json::json!({
        "ok": true,
        "browsers": {
            "chromium": {
                "supported": true,
                "note": "uses the browser-level Chrome DevTools Protocol websocket from DevToolsActivePort or --cdp"
            },
            "safari": {
                "supported": false,
                "note": "Safari WebDriver uses isolated automation sessions; signed-in normal-profile control is not exposed through a CDP-like API"
            }
        },
        "endpoints": endpoints,
        "selection": selection,
        "config": config::config_path().ok().map(|p| p.display().to_string()),
    })
}

async fn discover_chromium() -> Result<BrowserEndpoint> {
    if let Ok(path) = config::managed_devtools_file()
        && let Ok(endpoint) = managed_endpoint_from_devtools_file(&path).await
    {
        return Ok(endpoint);
    }

    for path in devtools_files() {
        if let Ok(endpoint) = endpoint_from_devtools_file(&path).await {
            return Ok(endpoint);
        }
    }

    for port in [9222_u16, 9229, 9333] {
        let endpoint = format!("http://127.0.0.1:{port}");
        if let Ok(found) = resolve_http(&endpoint).await {
            return Ok(found);
        }
    }

    Err(Error::BrowserNotFound)
}

async fn managed_endpoint_from_devtools_file(path: &Path) -> Result<BrowserEndpoint> {
    let endpoint = endpoint_from_devtools_file(path).await?;
    let port = endpoint
        .websocket_url
        .split(':')
        .nth(2)
        .and_then(|rest| rest.split('/').next())
        .ok_or_else(|| Error::InvalidArgument("invalid managed websocket URL".to_owned()))?;
    resolve_http(&format!("http://127.0.0.1:{port}"))
        .await
        .map(|found| BrowserEndpoint {
            source: endpoint.source,
            ..found
        })
}

async fn resolve_explicit(value: &str) -> Result<BrowserEndpoint> {
    if value.starts_with("ws://") || value.starts_with("wss://") {
        return Ok(BrowserEndpoint {
            backend: "chromium".to_owned(),
            websocket_url: value.to_owned(),
            source: "--cdp".to_owned(),
        });
    }
    if value.chars().all(|c| c.is_ascii_digit()) {
        return resolve_http(&format!("http://127.0.0.1:{value}")).await;
    }
    resolve_http(value).await
}

async fn resolve_http(value: &str) -> Result<BrowserEndpoint> {
    let base = Url::parse(value)?;
    let version_url = base.join("/json/version")?;
    let body = get_json(&version_url).await?;
    let websocket_url = body
        .get("webSocketDebuggerUrl")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| Error::Protocol {
            method: "/json/version".to_owned(),
            message: "missing webSocketDebuggerUrl".to_owned(),
        })?;
    Ok(BrowserEndpoint {
        backend: "chromium".to_owned(),
        websocket_url: websocket_url.to_owned(),
        source: value.to_owned(),
    })
}

const MAX_DISCOVERY_RESPONSE_BYTES: u64 = 1024 * 1024;

async fn get_json(url: &Url) -> Result<serde_json::Value> {
    if url.scheme() != "http" {
        return Err(http_error(
            url,
            "only plain HTTP CDP discovery is supported",
        ));
    }

    let host = url
        .host_str()
        .ok_or_else(|| http_error(url, "URL is missing a host"))?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| http_error(url, "URL is missing a port"))?;
    let host_display = if host.contains(':') {
        format!("[{host}]")
    } else {
        host.to_owned()
    };
    let host_header = match url.port() {
        Some(port) => format!("{host_display}:{port}"),
        None => host_display,
    };
    let request_target = match url.query() {
        Some(query) => format!("{}?{query}", url.path()),
        None => url.path().to_owned(),
    };

    let mut stream = TcpStream::connect((host, port))
        .await
        .map_err(|error| http_error(url, error))?;
    let request = format!(
        "GET {request_target} HTTP/1.1\r\nHost: {host_header}\r\nAccept: application/json\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| http_error(url, error))?;

    let mut response = Vec::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let count = stream
            .read(&mut buffer)
            .await
            .map_err(|error| http_error(url, error))?;
        if count == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..count]);
        if response.len() as u64 > MAX_DISCOVERY_RESPONSE_BYTES {
            return Err(http_error(url, "response exceeded 1 MiB"));
        }
        if http_response_is_complete(&response) {
            break;
        }
    }
    if response.len() as u64 > MAX_DISCOVERY_RESPONSE_BYTES {
        return Err(http_error(url, "response exceeded 1 MiB"));
    }

    let body = parse_http_response(url, &response)?;
    serde_json::from_slice(&body).map_err(|error| http_error(url, error))
}

fn http_response_is_complete(response: &[u8]) -> bool {
    let Some(header_end) = response.windows(4).position(|window| window == b"\r\n\r\n") else {
        return false;
    };
    let Ok(headers) = std::str::from_utf8(&response[..header_end]) else {
        return false;
    };
    let body_len = response.len().saturating_sub(header_end + 4);
    headers.split("\r\n").skip(1).any(|line| {
        line.split_once(':').is_some_and(|(name, value)| {
            name.eq_ignore_ascii_case("content-length")
                && value
                    .trim()
                    .parse::<usize>()
                    .is_ok_and(|length| body_len >= length)
        })
    })
}

fn parse_http_response(url: &Url, response: &[u8]) -> Result<Vec<u8>> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| http_error(url, "response did not contain HTTP headers"))?;
    let headers =
        std::str::from_utf8(&response[..header_end]).map_err(|error| http_error(url, error))?;
    let mut lines = headers.split("\r\n");
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .ok_or_else(|| http_error(url, "response had an invalid HTTP status"))?;
    if !(200..300).contains(&status) {
        return Err(http_error(url, format!("server returned HTTP {status}")));
    }

    let body = &response[header_end + 4..];
    let chunked = lines.any(|line| {
        line.split_once(':').is_some_and(|(name, value)| {
            name.eq_ignore_ascii_case("transfer-encoding")
                && value
                    .split(',')
                    .any(|encoding| encoding.trim().eq_ignore_ascii_case("chunked"))
        })
    });
    if chunked {
        decode_chunked(url, body)
    } else {
        Ok(body.to_vec())
    }
}

fn decode_chunked(url: &Url, mut input: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    loop {
        let line_end = input
            .windows(2)
            .position(|window| window == b"\r\n")
            .ok_or_else(|| http_error(url, "chunked response is missing a size line"))?;
        let size_text =
            std::str::from_utf8(&input[..line_end]).map_err(|error| http_error(url, error))?;
        let size = usize::from_str_radix(size_text.split(';').next().unwrap_or_default(), 16)
            .map_err(|error| http_error(url, error))?;
        input = &input[line_end + 2..];
        if size == 0 {
            return Ok(output);
        }
        let chunk_end = size
            .checked_add(2)
            .ok_or_else(|| http_error(url, "chunk size overflowed"))?;
        if input.len() < chunk_end || &input[size..chunk_end] != b"\r\n" {
            return Err(http_error(url, "chunked response ended early"));
        }
        output.extend_from_slice(&input[..size]);
        input = &input[chunk_end..];
    }
}

fn http_error(url: &Url, message: impl std::fmt::Display) -> Error {
    Error::Http {
        url: url.to_string(),
        message: message.to_string(),
    }
}

pub async fn endpoint_from_devtools_file(path: &Path) -> Result<BrowserEndpoint> {
    let raw = tokio::fs::read_to_string(path)
        .await
        .map_err(|source| Error::Io {
            path: path.display().to_string(),
            source,
        })?;
    let mut lines = raw.lines();
    let port = lines
        .next()
        .and_then(|value| value.trim().parse::<u16>().ok())
        .filter(|port| *port != 0)
        .ok_or_else(|| Error::InvalidArgument("invalid DevToolsActivePort port".to_owned()))?;
    let browser_path = lines
        .next()
        .map(str::trim)
        .ok_or_else(|| Error::InvalidArgument("missing browser websocket path".to_owned()))?;
    let valid_id = browser_path
        .strip_prefix("/devtools/browser/")
        .is_some_and(|id| {
            !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        });
    if !valid_id {
        return Err(Error::InvalidArgument(
            "invalid browser websocket path".to_owned(),
        ));
    }
    Ok(BrowserEndpoint {
        backend: "chromium".to_owned(),
        websocket_url: format!("ws://127.0.0.1:{port}{browser_path}"),
        source: path.display().to_string(),
    })
}

pub fn existing_chrome_profile() -> Result<PathBuf> {
    let base = dirs::config_dir().ok_or_else(|| {
        Error::InvalidArgument(
            "cannot resolve Chrome user-data directory; pass --profile".to_owned(),
        )
    })?;
    #[cfg(target_os = "macos")]
    let path = base.join("Google/Chrome");
    #[cfg(target_os = "windows")]
    let path = dirs::data_local_dir()
        .unwrap_or(base)
        .join("Google/Chrome/User Data");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let path = base.join("google-chrome");
    Ok(path)
}

fn devtools_files() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join("Library/Application Support/Google/Chrome/DevToolsActivePort"));
        paths
            .push(home.join("Library/Application Support/Google/Chrome Canary/DevToolsActivePort"));
        paths.push(home.join("Library/Application Support/Microsoft Edge/DevToolsActivePort"));
        paths.push(
            home.join("Library/Application Support/BraveSoftware/Brave-Browser/DevToolsActivePort"),
        );
        paths.push(home.join("Library/Application Support/Arc/User Data/DevToolsActivePort"));
        paths.push(home.join(".config/google-chrome/DevToolsActivePort"));
        paths.push(home.join(".config/chromium/DevToolsActivePort"));
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::{decode_chunked, get_json, http_response_is_complete, parse_http_response};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    use url::Url;

    #[tokio::test]
    async fn fetches_json_over_local_http() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 1024];
            let count = socket.read(&mut request).await.unwrap();
            assert!(request[..count].starts_with(b"GET /json/version HTTP/1.1\r\n"));
            socket
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}",
                )
                .await
                .unwrap();
        };
        let url = Url::parse(&format!("http://{address}/json/version")).unwrap();

        let ((), result) = tokio::join!(server, get_json(&url));

        assert_eq!(result.unwrap(), serde_json::json!({ "ok": true }));
    }

    #[test]
    fn parses_successful_http_response() {
        let url = Url::parse("http://127.0.0.1:9222/json/version").unwrap();
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\n\r\n{\"ok\":true}";

        assert_eq!(
            parse_http_response(&url, response).unwrap(),
            br#"{"ok":true}"#
        );
    }

    #[test]
    fn recognizes_a_complete_content_length_response() {
        assert!(http_response_is_complete(
            b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\n{\"ok\":true}"
        ));
        assert!(!http_response_is_complete(
            b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\n{\"ok\":"
        ));
    }

    #[test]
    fn decodes_chunked_response() {
        let url = Url::parse("http://127.0.0.1:9222/json/version").unwrap();

        assert_eq!(
            decode_chunked(&url, b"4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n").unwrap(),
            b"Wikipedia"
        );
    }

    #[test]
    fn rejects_unsuccessful_http_response() {
        let url = Url::parse("http://127.0.0.1:9222/json/version").unwrap();
        let response = b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";

        let error = parse_http_response(&url, response).unwrap_err();
        assert_eq!(error.code(), "http_error");
        assert!(error.to_string().contains("HTTP 404"));
    }

    #[tokio::test]
    async fn reports_invalid_http_json_as_http_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 1024];
            let _ = socket.read(&mut request).await.unwrap();
            socket
                .write_all(
                    b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\nnot JSON",
                )
                .await
                .unwrap();
        };
        let url = Url::parse(&format!("http://{address}/json/version")).unwrap();

        let ((), result) = tokio::join!(server, get_json(&url));
        let error = result.unwrap_err();

        assert_eq!(error.code(), "http_error");
    }
}
