use std::{collections::VecDeque, time::Duration};

use base64::Engine as _;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpStream;
use tokio::time::Instant;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async_with_config, tungstenite::protocol::Message,
};

use crate::error::{Error, Result};

pub(crate) type BrowserSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

const MAX_QUEUED_EVENTS: usize = 4096;
const MAX_QUEUED_BYTES: usize = 8 * 1024 * 1024;

struct QueuedEvent {
    value: Value,
    bytes: usize,
}

struct Navigation {
    frame: String,
    commit: NavigationCommit,
    deadline: Instant,
    committed: bool,
}

enum NavigationCommit {
    Loader(String),
    Changed {
        previous_loader: String,
        within_document_url: Option<String>,
    },
}

enum Socket {
    Direct(Box<BrowserSocket>),
    #[cfg(unix)]
    Shared(Box<WebSocketStream<tokio::net::UnixStream>>),
}

impl Socket {
    async fn send(
        &mut self,
        message: Message,
    ) -> std::result::Result<(), tokio_tungstenite::tungstenite::Error> {
        match self {
            Self::Direct(socket) => socket.send(message).await,
            #[cfg(unix)]
            Self::Shared(socket) => socket.send(message).await,
        }
    }

    async fn next(
        &mut self,
    ) -> Option<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>> {
        match self {
            Self::Direct(socket) => socket.next().await,
            #[cfg(unix)]
            Self::Shared(socket) => socket.next().await,
        }
    }
}

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
    socket: Socket,
    next_id: u64,
    queued: VecDeque<QueuedEvent>,
    queued_bytes: usize,
    navigation: Option<Navigation>,
    session_id: Option<String>,
    target_id: Option<String>,
    timeout: Duration,
    pub cache_scope: String,
}

impl CdpClient {
    pub async fn connect(websocket_url: &str, timeout_ms: u64) -> Result<Self> {
        let timeout = Duration::from_millis(timeout_ms);
        // Native CDP clients do not impersonate a web origin. Chrome owns the
        // approval handshake for its opt-in, everyday-browser endpoint.
        // CDP exchanges small request/response messages. Nagle buffering adds
        // latency without useful batching on this serialized control channel.
        let (socket, _) = tokio::time::timeout(
            timeout,
            connect_async_with_config(websocket_url, None, true),
        )
        .await
        .map_err(|_| Error::Timeout {
            operation: "websocket connect".to_owned(),
            timeout_ms,
        })??;
        Ok(Self {
            socket: Socket::Direct(Box::new(socket)),
            next_id: 1,
            queued: VecDeque::new(),
            queued_bytes: 0,
            navigation: None,
            session_id: None,
            target_id: None,
            timeout,
            cache_scope: websocket_url.to_owned(),
        })
    }

    #[cfg(unix)]
    pub(crate) fn into_browser_socket(self) -> BrowserSocket {
        let Socket::Direct(socket) = self.socket else {
            unreachable!("only a direct connection can start a helper")
        };
        *socket
    }

    #[cfg(unix)]
    pub async fn connect_session(
        session: &crate::config::BrowserSession,
        timeout_ms: u64,
    ) -> Result<Self> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        let stream = tokio::time::timeout_at(
            deadline,
            tokio::net::UnixStream::connect(session.directory.join("cdp")),
        )
        .await
        .map_err(|_| Error::BrowserBusy)?
        .map_err(|_| super::session::disconnected())?;
        let (socket, _) = tokio::time::timeout_at(
            deadline,
            tokio_tungstenite::client_async("ws://localhost/session", stream),
        )
        .await
        .map_err(|_| Error::BrowserBusy)?
        .map_err(|_| super::session::disconnected())?;
        Ok(Self {
            socket: Socket::Shared(Box::new(socket)),
            next_id: 1,
            queued: VecDeque::new(),
            queued_bytes: 0,
            navigation: None,
            session_id: None,
            target_id: None,
            timeout: Duration::from_millis(timeout_ms),
            cache_scope: session.endpoint.clone(),
        })
    }

    #[cfg(not(unix))]
    pub async fn connect_session(_: &crate::config::BrowserSession, _: u64) -> Result<Self> {
        Err(super::session::unsupported())
    }

    fn socket_error(&self, error: tokio_tungstenite::tungstenite::Error) -> Error {
        match self.socket {
            Socket::Direct(_) => Error::WebSocket(error),
            #[cfg(unix)]
            Socket::Shared(_) => super::session::disconnected(),
        }
    }

    fn socket_closed(&self, method: &str) -> Error {
        match self.socket {
            Socket::Direct(_) => Error::Protocol {
                method: method.to_owned(),
                message: "websocket closed".to_owned(),
            },
            #[cfg(unix)]
            Socket::Shared(_) => super::session::disconnected(),
        }
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

    pub async fn verify(&mut self) -> Result<()> {
        self.send_browser("Browser.getVersion", json!({})).await?;
        Ok(())
    }

    pub async fn close_browser(&mut self) -> Result<()> {
        self.send_browser("Browser.close", json!({})).await?;
        Ok(())
    }

    pub async fn create_target(&mut self, url: &str) -> Result<TargetInfo> {
        let result = self
            .send_browser("Target.createTarget", create_target_params(url))
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
        self.navigation = None;
        self.send_page("Page.enable", json!({})).await?;
        Ok(())
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        self.start_navigation(url).await?;
        self.wait_ready().await
    }

    pub async fn start_navigation(&mut self, url: &str) -> Result<()> {
        self.navigation = None;
        let deadline = Instant::now() + self.timeout;
        let result = self
            .send_page_until("Page.navigate", json!({ "url": url }), deadline)
            .await?;
        if let Some(message) = result
            .get("errorText")
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
        {
            return Err(Error::Protocol {
                method: "Page.navigate".to_owned(),
                message: message.to_owned(),
            });
        }
        if let (Some(frame), Some(loader)) = (
            result.get("frameId").and_then(Value::as_str),
            result.get("loaderId").and_then(Value::as_str),
        ) {
            self.navigation = Some(Navigation {
                frame: frame.to_owned(),
                commit: NavigationCommit::Loader(loader.to_owned()),
                deadline,
                committed: false,
            });
        }
        Ok(())
    }

    pub async fn reload(&mut self) -> Result<Value> {
        let deadline = Instant::now() + self.timeout;
        self.prepare_document_change(deadline, None).await?;
        self.send_page_until("Page.reload", json!({}), deadline)
            .await?;
        self.wait_ready().await?;
        self.page_info().await
    }

    pub async fn history(&mut self, delta: i64) -> Result<Value> {
        if delta == 0 {
            return self.reload().await;
        }
        let deadline = Instant::now() + self.timeout;
        let history = self
            .send_page_until("Page.getNavigationHistory", json!({}), deadline)
            .await?;
        let index = history
            .get("currentIndex")
            .and_then(Value::as_i64)
            .and_then(|index| index.checked_add(delta));
        let entry = index
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| history.get("entries")?.as_array()?.get(index));
        let Some(entry) = entry else {
            return self.page_info().await;
        };
        let id = entry
            .get("id")
            .and_then(Value::as_i64)
            .ok_or_else(|| Error::Protocol {
                method: "Page.getNavigationHistory".to_owned(),
                message: "missing history entry id".to_owned(),
            })?;
        let url = entry.get("url").and_then(Value::as_str).map(str::to_owned);
        self.prepare_document_change(deadline, url).await?;
        self.send_page_until(
            "Page.navigateToHistoryEntry",
            json!({"entryId":id}),
            deadline,
        )
        .await?;
        self.wait_ready().await?;
        self.page_info().await
    }

    async fn prepare_document_change(
        &mut self,
        deadline: Instant,
        within_document_url: Option<String>,
    ) -> Result<()> {
        self.navigation = None;
        let tree = self
            .send_page_until("Page.getFrameTree", json!({}), deadline)
            .await?;
        let frame = tree.pointer("/frameTree/frame/id").and_then(Value::as_str);
        let loader = tree
            .pointer("/frameTree/frame/loaderId")
            .and_then(Value::as_str);
        let (Some(frame), Some(loader)) = (frame, loader) else {
            return Err(Error::Protocol {
                method: "Page.getFrameTree".to_owned(),
                message: "missing main frame identity".to_owned(),
            });
        };
        // Ignore events from previous operations. New events are observed as
        // soon as the navigation command is sent, even before its response.
        self.navigation = Some(Navigation {
            frame: frame.to_owned(),
            commit: NavigationCommit::Changed {
                previous_loader: loader.to_owned(),
                within_document_url,
            },
            deadline,
            committed: false,
        });
        Ok(())
    }

    pub async fn wait_ready(&mut self) -> Result<()> {
        self.wait_for_js(
            "document.readyState === 'interactive' || document.readyState === 'complete'",
        )
        .await?;
        Ok(())
    }

    pub async fn wait_for_js(&mut self, expression: &str) -> Result<Value> {
        let deadline = self.navigation.as_ref().map_or_else(
            || Instant::now() + self.timeout,
            |navigation| navigation.deadline,
        );
        self.wait_navigation_commit(deadline).await?;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(self.timeout_error("page condition"));
            }
            let script = wait_script(expression, remaining.as_millis());
            match self.evaluate_until(&script, true, deadline).await {
                Ok(value) if value.as_bool() == Some(true) => {
                    self.navigation = None;
                    return Ok(json!({ "ok": true }));
                }
                Ok(_) => return Err(self.timeout_error("page condition")),
                Err(error) if navigation_context_lost(&error) => {
                    // A redirect can destroy the context while the promise is pending.
                    // Back off locally; do not hide script or other protocol failures.
                    tokio::time::sleep_until(
                        (Instant::now() + Duration::from_millis(10)).min(deadline),
                    )
                    .await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    pub async fn evaluate(&mut self, expression: &str, return_by_value: bool) -> Result<Value> {
        self.evaluate_until(expression, return_by_value, Instant::now() + self.timeout)
            .await
    }

    async fn evaluate_until(
        &mut self,
        expression: &str,
        return_by_value: bool,
        deadline: Instant,
    ) -> Result<Value> {
        let result = self
            .send_page_until(
                "Runtime.evaluate",
                json!({
                    "expression": expression,
                    "awaitPromise": true,
                    "returnByValue": return_by_value,
                    "userGesture": true,
                }),
                deadline,
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
        let deadline = Instant::now() + duration;
        while Instant::now() < deadline {
            match self.receive_until("record events", deadline).await {
                Ok(value) => self.queue_event(value),
                Err(_) => break,
            }
        }
        self.queued_bytes = 0;
        self.queued.drain(..).map(|event| event.value).collect()
    }

    pub async fn send_page(&mut self, method: &str, params: Value) -> Result<Value> {
        self.send_page_until(method, params, Instant::now() + self.timeout)
            .await
    }

    async fn send_page_until(
        &mut self,
        method: &str,
        params: Value,
        deadline: Instant,
    ) -> Result<Value> {
        let session_id = self.session_id.clone().ok_or_else(|| Error::Protocol {
            method: method.to_owned(),
            message: "no attached target session".to_owned(),
        })?;
        self.send_until(Some(&session_id), method, params, deadline)
            .await
    }

    async fn send_browser(&mut self, method: &str, params: Value) -> Result<Value> {
        self.send_until(None, method, params, Instant::now() + self.timeout)
            .await
    }

    async fn send_until(
        &mut self,
        session_id: Option<&str>,
        method: &str,
        params: Value,
        deadline: Instant,
    ) -> Result<Value> {
        if Instant::now() >= deadline {
            return Err(self.timeout_error(method));
        }
        let id = self.next_id;
        self.next_id += 1;
        let mut request = json!({ "id": id, "method": method, "params": params });
        if let Some(session_id) = session_id {
            request["sessionId"] = Value::String(session_id.to_owned());
        }
        tokio::time::timeout_at(
            deadline,
            self.socket.send(Message::Text(request.to_string().into())),
        )
        .await
        .map_err(|_| self.timeout_error(method))?
        .map_err(|error| self.socket_error(error))?;
        loop {
            let value = self.receive_until(method, deadline).await?;
            if value.get("id").and_then(Value::as_u64) == Some(id) {
                return parse_response(method, &value);
            }
            // This client has one outstanding request. Responses to timed-out
            // requests can never satisfy a future call and must not accumulate.
            self.queue_event(value);
        }
    }

    fn timeout_error(&self, operation: &str) -> Error {
        Error::Timeout {
            operation: operation.to_owned(),
            timeout_ms: self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
        }
    }

    async fn receive_until(&mut self, method: &str, deadline: Instant) -> Result<Value> {
        loop {
            if Instant::now() >= deadline {
                return Err(self.timeout_error(method));
            }
            let next = tokio::time::timeout_at(deadline, self.socket.next())
                .await
                .map_err(|_| self.timeout_error(method))?;
            let Some(message) = next else {
                return Err(self.socket_closed(method));
            };
            match message.map_err(|error| self.socket_error(error))? {
                Message::Text(text) => {
                    return Ok(serde_json::from_str(&text)?);
                }
                Message::Close(_) => {
                    return Err(self.socket_closed(method));
                }
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
            }
        }
    }

    fn queue_event(&mut self, value: Value) {
        if value.get("id").is_some() || value.get("method").is_none() {
            return;
        }
        if let Some(navigation) = &mut self.navigation {
            navigation.committed |=
                navigation_matches(navigation, &value, self.session_id.as_deref());
        }
        let bytes = value.to_string().len();
        if bytes > MAX_QUEUED_BYTES {
            return;
        }
        while self.queued.len() >= MAX_QUEUED_EVENTS || self.queued_bytes + bytes > MAX_QUEUED_BYTES
        {
            if let Some(event) = self.queued.pop_front() {
                self.queued_bytes -= event.bytes;
            }
        }
        self.queued_bytes += bytes;
        self.queued.push_back(QueuedEvent { value, bytes });
    }

    async fn wait_navigation_commit(&mut self, deadline: Instant) -> Result<()> {
        if let Some(navigation) = &mut self.navigation
            && matches!(navigation.commit, NavigationCommit::Loader(_))
        {
            navigation.committed |= self.queued.iter().any(|event| {
                navigation_matches(navigation, &event.value, self.session_id.as_deref())
            });
        }
        while self
            .navigation
            .as_ref()
            .is_some_and(|navigation| !navigation.committed)
        {
            let event = self.receive_until("navigation commit", deadline).await?;
            self.queue_event(event);
        }
        Ok(())
    }
}

fn navigation_matches(navigation: &Navigation, event: &Value, session: Option<&str>) -> bool {
    if event.get("sessionId").and_then(Value::as_str) != session {
        return false;
    }
    let method = event.get("method").and_then(Value::as_str);
    if method == Some("Page.frameNavigated")
        && event.pointer("/params/frame/id").and_then(Value::as_str) == Some(&navigation.frame)
    {
        let loader = event
            .pointer("/params/frame/loaderId")
            .and_then(Value::as_str);
        return match &navigation.commit {
            NavigationCommit::Loader(expected) => loader == Some(expected),
            NavigationCommit::Changed {
                previous_loader, ..
            } => loader.is_some_and(|loader| loader != previous_loader),
        };
    }
    matches!(&navigation.commit, NavigationCommit::Changed { within_document_url: Some(url), .. }
        if method == Some("Page.navigatedWithinDocument")
        && event.pointer("/params/frameId").and_then(Value::as_str) == Some(&navigation.frame)
        && event.pointer("/params/url").and_then(Value::as_str) == Some(url))
}

fn navigation_context_lost(error: &Error) -> bool {
    matches!(error, Error::Protocol { method, message } if method == "Runtime.evaluate" && (
        message.contains("Execution context was destroyed") || message.contains("Cannot find context with specified id")
    ))
}

fn wait_script(expression: &str, timeout_ms: u128) -> String {
    // Observe DOM changes immediately. The fallback timer also covers predicates
    // involving application state with no mutation, without CDP round trips.
    format!(
        r"new Promise((resolve, reject) => {{
        let observer, interval, timer, pending = false, finished = false;
        const finish = (value, error) => {{
            if (finished) return;
            finished = true;
            observer?.disconnect(); clearInterval(interval); clearTimeout(timer);
            document.removeEventListener('readystatechange', check);
            error ? reject(error) : resolve(value);
        }};
        const check = () => {{
            if (finished || pending) return;
            try {{
                const value = ({expression});
                if (value === true) finish(true);
                else if (value && typeof value.then === 'function') {{
                    pending = true;
                    Promise.resolve(value).then(value => {{
                        pending = false;
                        if (value === true) finish(true);
                    }}, error => finish(false, error));
                }}
            }} catch (error) {{ finish(false, error); }}
        }};
        observer = new MutationObserver(check);
        observer.observe(document, {{ subtree: true, childList: true, attributes: true, characterData: true }});
        document.addEventListener('readystatechange', check);
        interval = setInterval(check, 50);
        timer = setTimeout(() => finish(false), {timeout_ms});
        check();
    }})"
    )
}

fn create_target_params(url: &str) -> Value {
    // Creating a target in the foreground can switch the active Chrome window
    // away from the agent that started the search. Background targets remain
    // fully controllable over CDP without taking keyboard focus.
    json!({ "url": url, "background": true })
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

#[cfg(test)]
mod tests {
    use super::*;

    async fn mock_client(timeout: u64) -> (CdpClient, WebSocketStream<tokio::net::TcpStream>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            tokio_tungstenite::accept_async(stream).await.unwrap()
        });
        let mut client = CdpClient::connect(&format!("ws://{address}"), timeout)
            .await
            .unwrap();
        client.session_id = Some("page-session".to_owned());
        (client, peer.await.unwrap())
    }

    async fn call(socket: &mut WebSocketStream<tokio::net::TcpStream>) -> Value {
        serde_json::from_str(socket.next().await.unwrap().unwrap().to_text().unwrap()).unwrap()
    }

    async fn reply(socket: &mut WebSocketStream<tokio::net::TcpStream>, value: Value) {
        socket
            .send(Message::Text(value.to_string().into()))
            .await
            .unwrap();
    }

    fn navigation_event(loader: &str) -> Value {
        json!({"method":"Page.frameNavigated", "sessionId":"page-session", "params":{"frame":{"id":"main", "loaderId":loader}}})
    }

    async fn finish_ready_and_info(socket: &mut WebSocketStream<tokio::net::TcpStream>) {
        let ready = call(socket).await;
        assert_eq!(ready["method"], "Runtime.evaluate");
        assert!(
            ready["params"]["expression"]
                .as_str()
                .unwrap()
                .contains("MutationObserver")
        );
        reply(
            socket,
            json!({"id":ready["id"],"result":{"result":{"value":true}}}),
        )
        .await;
        let info = call(socket).await;
        assert_eq!(info["method"], "Runtime.evaluate");
        reply(
            socket,
            json!({"id":info["id"],"result":{"result":{"value":{"url":"https://example.test/"}}}}),
        )
        .await;
    }

    #[tokio::test]
    async fn direct_transport_disables_nagle_buffering() {
        let (client, _socket) = mock_client(1000).await;
        let Socket::Direct(socket) = client.socket else {
            panic!("expected direct transport")
        };
        let MaybeTlsStream::Plain(stream) = socket.get_ref() else {
            panic!("expected plain test stream")
        };
        assert!(stream.nodelay().unwrap());
    }

    #[tokio::test]
    async fn reload_ignores_older_events_and_waits_for_new_document() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let tree = call(&mut socket).await;
            assert_eq!(tree["method"], "Page.getFrameTree");
            reply(&mut socket, navigation_event("older-document")).await;
            reply(&mut socket, json!({"id":tree["id"],"result":{"frameTree":{"frame":{"id":"main","loaderId":"current-document"}}}})).await;
            let reload = call(&mut socket).await;
            assert_eq!(reload["method"], "Page.reload");
            reply(&mut socket, json!({"id":reload["id"],"result":{}})).await;
            reply(&mut socket, navigation_event("current-document")).await;
            assert!(
                tokio::time::timeout(Duration::from_millis(30), socket.next())
                    .await
                    .is_err()
            );
            reply(&mut socket, navigation_event("reloaded-document")).await;
            finish_ready_and_info(&mut socket).await;
        });
        client.reload().await.unwrap();
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn history_waits_for_same_document_and_cross_document_commits() {
        for same_document in [false, true] {
            let (mut client, mut socket) = mock_client(1000).await;
            let peer = tokio::spawn(async move {
                let history = call(&mut socket).await;
                assert_eq!(history["method"], "Page.getNavigationHistory");
                reply(&mut socket, json!({"id":history["id"],"result":{"currentIndex":1,"entries":[{"id":3,"url":"https://example.test/#old"},{"id":7,"url":"https://example.test/#new"}]}})).await;
                let tree = call(&mut socket).await;
                assert_eq!(tree["method"], "Page.getFrameTree");
                reply(&mut socket, json!({"id":tree["id"],"result":{"frameTree":{"frame":{"id":"main","loaderId":"current-document"}}}})).await;
                let navigate = call(&mut socket).await;
                assert_eq!(navigate["method"], "Page.navigateToHistoryEntry");
                assert_eq!(navigate["params"]["entryId"], 3);
                reply(&mut socket, json!({"id":navigate["id"],"result":{}})).await;
                assert!(
                    tokio::time::timeout(Duration::from_millis(30), socket.next())
                        .await
                        .is_err()
                );
                let event = if same_document {
                    json!({"method":"Page.navigatedWithinDocument","sessionId":"page-session","params":{"frameId":"main","url":"https://example.test/#old"}})
                } else {
                    navigation_event("previous-document")
                };
                reply(&mut socket, event).await;
                finish_ready_and_info(&mut socket).await;
            });
            client.history(-1).await.unwrap();
            peer.await.unwrap();
        }
    }

    #[tokio::test]
    async fn history_outside_available_entries_is_a_noop() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let history = call(&mut socket).await;
            reply(&mut socket, json!({"id":history["id"],"result":{"currentIndex":0,"entries":[{"id":1,"url":"https://example.test/"}]}})).await;
            let info = call(&mut socket).await;
            assert_eq!(info["method"], "Runtime.evaluate");
            assert!(
                !info["params"]["expression"]
                    .as_str()
                    .unwrap()
                    .contains("MutationObserver")
            );
            reply(&mut socket, json!({"id":info["id"],"result":{"result":{"value":{"url":"https://example.test/"}}}})).await;
        });
        client.history(-1).await.unwrap();
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn event_flood_does_not_extend_command_deadline() {
        let (mut client, mut socket) = mock_client(70).await;
        let peer = tokio::spawn(async move {
            let _ = call(&mut socket).await;
            loop {
                if socket
                    .send(Message::Text(
                        json!({"method":"Network.dataReceived","params":{}})
                            .to_string()
                            .into(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(2)).await;
            }
        });
        let started = Instant::now();
        let result = tokio::time::timeout(Duration::from_millis(350), client.verify())
            .await
            .expect("event flood must not keep the operation alive");
        assert!(matches!(result, Err(Error::Timeout { .. })));
        assert!(started.elapsed() < Duration::from_millis(300));
        peer.abort();
    }

    #[tokio::test]
    async fn queued_events_are_bounded_and_recording_consumes_them() {
        let (mut client, _socket) = mock_client(1000).await;
        for index in 0..MAX_QUEUED_EVENTS + 100 {
            client.queue_event(
                json!({"method":"Network.requestWillBeSent", "params":{"index":index}}),
            );
        }
        client.queue_event(json!({"id":999,"result":{}}));
        assert_eq!(client.queued.len(), MAX_QUEUED_EVENTS);
        let events = client.drain_events_for(Duration::ZERO).await;
        assert_eq!(events.len(), MAX_QUEUED_EVENTS);
        assert_eq!(events[0]["params"]["index"], 100);
        assert_eq!(client.queued_bytes, 0);
        assert!(client.drain_events_for(Duration::ZERO).await.is_empty());
        let payload = "x".repeat(MAX_QUEUED_BYTES / 3);
        for _ in 0..5 {
            client.queue_event(json!({"method":"Log.entryAdded","params":{"text":payload}}));
        }
        assert!(client.queued_bytes <= MAX_QUEUED_BYTES);
        assert!(client.queued.len() <= 2);
    }

    #[tokio::test]
    async fn navigation_waits_for_new_loader_before_evaluating() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let request = call(&mut socket).await;
            assert_eq!(request["method"], "Page.navigate");
            reply(&mut socket, navigation_event("old-document")).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"result":{"frameId":"main","loaderId":"new-document"}}),
            )
            .await;
            // No readyState evaluation may run in the old about:blank document.
            assert!(
                tokio::time::timeout(Duration::from_millis(30), socket.next())
                    .await
                    .is_err()
            );
            reply(&mut socket, navigation_event("new-document")).await;
            let evaluate = call(&mut socket).await;
            assert_eq!(evaluate["method"], "Runtime.evaluate");
            let expression = evaluate["params"]["expression"].as_str().unwrap();
            assert!(expression.contains("MutationObserver"));
            reply(
                &mut socket,
                json!({"id":evaluate["id"],"result":{"result":{"value":true}}}),
            )
            .await;
        });
        client.navigate("https://example.test/").await.unwrap();
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn commit_event_before_navigation_response_is_retained() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let request = call(&mut socket).await;
            reply(&mut socket, navigation_event("new-document")).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"result":{"frameId":"main","loaderId":"new-document"}}),
            )
            .await;
            let evaluate = call(&mut socket).await;
            reply(
                &mut socket,
                json!({"id":evaluate["id"],"result":{"result":{"value":true}}}),
            )
            .await;
        });
        client.navigate("https://example.test/").await.unwrap();
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn navigation_errors_are_not_reported_as_ready() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let request = call(&mut socket).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"result":{"errorText":"net::ERR_NAME_NOT_RESOLVED"}}),
            )
            .await;
        });
        assert!(
            matches!(client.navigate("https://invalid.test/").await, Err(Error::Protocol { method, .. }) if method == "Page.navigate")
        );
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn waits_retry_destroyed_contexts_but_propagate_javascript_errors() {
        let (mut client, mut socket) = mock_client(1000).await;
        let peer = tokio::spawn(async move {
            let request = call(&mut socket).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"error":{"message":"Execution context was destroyed."}}),
            )
            .await;
            let request = call(&mut socket).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"result":{"exceptionDetails":{"text":"SyntaxError"}}}),
            )
            .await;
        });
        assert!(matches!(
            client.wait_for_js("missing()").await,
            Err(Error::JavaScript(_))
        ));
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn navigation_and_readiness_share_one_deadline() {
        let (mut client, mut socket) = mock_client(100).await;
        let peer = tokio::spawn(async move {
            let request = call(&mut socket).await;
            tokio::time::sleep(Duration::from_millis(60)).await;
            reply(
                &mut socket,
                json!({"id":request["id"],"result":{"frameId":"main","loaderId":"new-document"}}),
            )
            .await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        let started = Instant::now();
        assert!(matches!(
            client.navigate("https://example.test/").await,
            Err(Error::Timeout { .. })
        ));
        assert!(started.elapsed() < Duration::from_millis(150));
        peer.abort();
    }

    #[tokio::test]
    async fn browser_shutdown_uses_the_graceful_protocol_command() {
        use futures_util::{SinkExt as _, StreamExt as _};
        use tokio_tungstenite::tungstenite::Message;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let browser = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
            let message = socket.next().await.unwrap().unwrap();
            let call: serde_json::Value = serde_json::from_str(message.to_text().unwrap()).unwrap();
            assert_eq!(call["method"], "Browser.close");
            assert!(call.get("sessionId").is_none());
            socket
                .send(Message::Text(
                    serde_json::json!({"id":call["id"],"result":{}})
                        .to_string()
                        .into(),
                ))
                .await
                .unwrap();
        });
        let mut client = super::CdpClient::connect(&format!("ws://{address}"), 1000)
            .await
            .unwrap();
        client.close_browser().await.unwrap();
        browser.await.unwrap();
    }

    #[test]
    fn creates_targets_without_stealing_focus() {
        assert_eq!(
            create_target_params("about:blank"),
            serde_json::json!({
                "url": "about:blank",
                "background": true,
            })
        );
    }
}
