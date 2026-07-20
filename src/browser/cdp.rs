use std::{collections::VecDeque, time::Duration};

use base64::Engine as _;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub attached: bool,
}

pub struct CdpClient {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    next_id: u64,
    queued: VecDeque<Value>,
    session_id: Option<String>,
    target_id: Option<String>,
    timeout: Duration,
}

impl CdpClient {
    pub async fn connect(websocket_url: &str, timeout_ms: u64) -> Result<Self> {
        let timeout = Duration::from_millis(timeout_ms);
        let (socket, _) = tokio::time::timeout(timeout, connect_async(websocket_url))
            .await
            .map_err(|_| Error::Timeout {
                operation: "websocket connect".to_owned(),
                timeout_ms,
            })??;
        Ok(Self {
            socket,
            next_id: 1,
            queued: VecDeque::new(),
            session_id: None,
            target_id: None,
            timeout,
        })
    }

    pub async fn attach_or_create(&mut self, target_id: Option<&str>) -> Result<TargetInfo> {
        let target =
            if let Some(target_id) = target_id {
                self.targets()
                    .await?
                    .into_iter()
                    .find(|target| target.target_id == target_id)
                    .ok_or_else(|| Error::TargetNotFound(target_id.to_owned()))?
            } else if let Some(target) = self.targets().await?.into_iter().find(|target| {
                target.target_type == "page" && !target.url.starts_with("devtools://")
            }) {
                target
            } else {
                self.create_target("about:blank").await?
            };
        self.attach(&target.target_id).await?;
        Ok(target)
    }

    pub async fn targets(&mut self) -> Result<Vec<TargetInfo>> {
        let value = self
            .send_browser("Target.getTargets", json!({}))
            .await?
            .get("targetInfos")
            .cloned()
            .unwrap_or_else(|| json!([]));
        Ok(serde_json::from_value(value)?)
    }

    pub async fn create_target(&mut self, url: &str) -> Result<TargetInfo> {
        let result = self
            .send_browser("Target.createTarget", json!({ "url": url }))
            .await?;
        let target_id = result
            .get("targetId")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::Protocol {
                method: "Target.createTarget".to_owned(),
                message: "missing targetId".to_owned(),
            })?
            .to_owned();
        self.targets()
            .await?
            .into_iter()
            .find(|target| target.target_id == target_id)
            .ok_or(Error::TargetNotFound(target_id))
    }

    pub async fn close_target(&mut self, target_id: &str) -> Result<Value> {
        self.send_browser("Target.closeTarget", json!({ "targetId": target_id }))
            .await
    }

    pub fn target_id(&self) -> Option<&str> {
        self.target_id.as_deref()
    }

    pub async fn attach(&mut self, target_id: &str) -> Result<()> {
        let result = self
            .send_browser(
                "Target.attachToTarget",
                json!({ "targetId": target_id, "flatten": true }),
            )
            .await?;
        let session_id = result
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::Protocol {
                method: "Target.attachToTarget".to_owned(),
                message: "missing sessionId".to_owned(),
            })?
            .to_owned();
        self.session_id = Some(session_id);
        self.target_id = Some(target_id.to_owned());
        let _ = self.send_page("Page.enable", json!({})).await;
        let _ = self.send_page("Runtime.enable", json!({})).await;
        Ok(())
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        self.send_page("Page.navigate", json!({ "url": url }))
            .await?;
        self.wait_ready().await
    }

    pub async fn reload(&mut self) -> Result<Value> {
        self.send_page("Page.reload", json!({})).await?;
        self.wait_ready().await?;
        self.page_info().await
    }

    pub async fn history(&mut self, delta: i64) -> Result<Value> {
        self.evaluate(&format!("history.go({delta}); true"), true)
            .await?;
        self.wait_ready().await?;
        self.page_info().await
    }

    pub async fn wait_ready(&mut self) -> Result<()> {
        let attempts = (self.timeout.as_millis() / 250).max(1);
        for _ in 0..attempts {
            let value = self
                .evaluate("document.readyState", true)
                .await
                .unwrap_or_else(|_| Value::String("loading".to_owned()));
            if value
                .as_str()
                .is_some_and(|state| state == "interactive" || state == "complete")
            {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        Err(Error::Timeout {
            operation: "document.readyState".to_owned(),
            timeout_ms: self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
        })
    }

    pub async fn wait_for_js(&mut self, expression: &str) -> Result<Value> {
        let attempts = (self.timeout.as_millis() / 250).max(1);
        for _ in 0..attempts {
            let value = self.evaluate(expression, true).await?;
            if value.as_bool().unwrap_or(false) {
                return Ok(json!({ "ok": true }));
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        Err(Error::Timeout {
            operation: expression.to_owned(),
            timeout_ms: self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
        })
    }

    pub async fn evaluate(&mut self, expression: &str, return_by_value: bool) -> Result<Value> {
        let result = self
            .send_page(
                "Runtime.evaluate",
                json!({
                    "expression": expression,
                    "awaitPromise": true,
                    "returnByValue": return_by_value,
                    "userGesture": true,
                }),
            )
            .await?;
        if let Some(details) = result.get("exceptionDetails") {
            return Err(Error::JavaScript(details.to_string()));
        }
        let remote = result.get("result").cloned().unwrap_or_else(|| json!({}));
        Ok(remote.get("value").cloned().unwrap_or(remote))
    }

    pub async fn page_info(&mut self) -> Result<Value> {
        self.evaluate(
            "({ url: location.href, title: document.title, readyState: document.readyState })",
            true,
        )
        .await
    }

    pub async fn screenshot(&mut self, full_page: bool) -> Result<Vec<u8>> {
        let mut params = json!({ "format": "png", "fromSurface": true });
        if full_page {
            let metrics = self.send_page("Page.getLayoutMetrics", json!({})).await?;
            if let Some(size) = metrics.get("cssContentSize") {
                params["clip"] = json!({
                    "x": 0,
                    "y": 0,
                    "width": size.get("width").and_then(Value::as_f64).unwrap_or(1280.0),
                    "height": size.get("height").and_then(Value::as_f64).unwrap_or(720.0),
                    "scale": 1,
                });
            }
        }
        let result = self.send_page("Page.captureScreenshot", params).await?;
        decode_data(result.get("data"), "Page.captureScreenshot")
    }

    pub async fn pdf(&mut self) -> Result<Vec<u8>> {
        let result = self
            .send_page("Page.printToPDF", json!({ "printBackground": true }))
            .await?;
        decode_data(result.get("data"), "Page.printToPDF")
    }

    pub async fn mhtml(&mut self) -> Result<String> {
        let result = self
            .send_page("Page.captureSnapshot", json!({ "format": "mhtml" }))
            .await?;
        result
            .get("data")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::Protocol {
                method: "Page.captureSnapshot".to_owned(),
                message: "missing data".to_owned(),
            })
    }

    pub async fn press(&mut self, key: &str) -> Result<Value> {
        self.send_page(
            "Input.dispatchKeyEvent",
            json!({ "type": "keyDown", "text": key }),
        )
        .await?;
        self.send_page(
            "Input.dispatchKeyEvent",
            json!({ "type": "keyUp", "text": key }),
        )
        .await
    }

    pub async fn enable_recording(&mut self) -> Result<()> {
        self.send_page("Network.enable", json!({})).await?;
        self.send_page("Log.enable", json!({})).await?;
        Ok(())
    }

    pub async fn drain_events_for(&mut self, duration: Duration) -> Vec<Value> {
        let start = std::time::Instant::now();
        let mut events = Vec::new();
        while start.elapsed() < duration {
            match tokio::time::timeout(Duration::from_millis(200), self.socket.next()).await {
                Ok(Some(Ok(Message::Text(text)))) => {
                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                        events.push(value);
                    }
                }
                Ok(Some(Ok(Message::Frame(_) | Message::Close(_))) | None) => {
                    break;
                }
                Ok(Some(Ok(Message::Binary(_) | Message::Ping(_) | Message::Pong(_)) | Err(_)))
                | Err(_) => {}
            }
        }
        events
    }

    pub async fn send_page(&mut self, method: &str, params: Value) -> Result<Value> {
        let session_id = self.session_id.clone().ok_or_else(|| Error::Protocol {
            method: method.to_owned(),
            message: "no attached target session".to_owned(),
        })?;
        self.send(Some(&session_id), method, params).await
    }

    async fn send_browser(&mut self, method: &str, params: Value) -> Result<Value> {
        self.send(None, method, params).await
    }

    async fn send(
        &mut self,
        session_id: Option<&str>,
        method: &str,
        params: Value,
    ) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let mut request = json!({ "id": id, "method": method, "params": params });
        if let Some(session_id) = session_id {
            request["sessionId"] = Value::String(session_id.to_owned());
        }
        self.socket
            .send(Message::Text(request.to_string().into()))
            .await?;

        if let Some(pos) = self
            .queued
            .iter()
            .position(|event| event.get("id").and_then(Value::as_u64) == Some(id))
        {
            let response = self.queued.remove(pos).expect("position exists");
            return parse_response(method, &response);
        }

        loop {
            let next = tokio::time::timeout(self.timeout, self.socket.next())
                .await
                .map_err(|_| Error::Timeout {
                    operation: method.to_owned(),
                    timeout_ms: self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
                })?;
            let Some(message) = next else {
                return Err(Error::Protocol {
                    method: method.to_owned(),
                    message: "websocket closed".to_owned(),
                });
            };
            match message? {
                Message::Text(text) => {
                    let value: Value = serde_json::from_str(&text)?;
                    if value.get("id").and_then(Value::as_u64) == Some(id) {
                        return parse_response(method, &value);
                    }
                    self.queued.push_back(value);
                }
                Message::Close(_) => {
                    return Err(Error::Protocol {
                        method: method.to_owned(),
                        message: "websocket closed".to_owned(),
                    });
                }
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
            }
        }
    }
}

fn parse_response(method: &str, response: &Value) -> Result<Value> {
    if let Some(error) = response.get("error") {
        let message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("unknown protocol error")
            .to_owned();
        return Err(Error::Protocol {
            method: method.to_owned(),
            message,
        });
    }
    Ok(response.get("result").cloned().unwrap_or_else(|| json!({})))
}

fn decode_data(value: Option<&Value>, method: &str) -> Result<Vec<u8>> {
    let data = value
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Protocol {
            method: method.to_owned(),
            message: "missing data".to_owned(),
        })?;
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|err| Error::Protocol {
            method: method.to_owned(),
            message: err.to_string(),
        })
}
