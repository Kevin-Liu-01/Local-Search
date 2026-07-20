use std::path::{Path, PathBuf};

use serde::Serialize;
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
    let mut endpoints = Vec::new();
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
        "config": config::config_path().ok().map(|p| p.display().to_string()),
    })
}

async fn discover_chromium() -> Result<BrowserEndpoint> {
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
    let body: serde_json::Value = reqwest::get(version_url).await?.json().await?;
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

async fn endpoint_from_devtools_file(path: &Path) -> Result<BrowserEndpoint> {
    let raw = tokio::fs::read_to_string(path)
        .await
        .map_err(|source| Error::Io {
            path: path.display().to_string(),
            source,
        })?;
    let mut lines = raw.lines();
    let port = lines
        .next()
        .ok_or_else(|| Error::InvalidArgument("empty DevToolsActivePort".to_owned()))?;
    let browser_path = lines
        .next()
        .ok_or_else(|| Error::InvalidArgument("missing browser websocket path".to_owned()))?;
    Ok(BrowserEndpoint {
        backend: "chromium".to_owned(),
        websocket_url: format!("ws://127.0.0.1:{port}{browser_path}"),
        source: path.display().to_string(),
    })
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
