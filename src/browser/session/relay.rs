//! A single Chrome transport shared by isolated, serialized command leases.
//! Request IDs never repeat, late replies are discarded, and page sessions are
//! detached on lease release. The control socket can revoke an active lease.

use crate::{
    browser::cdp::BrowserSocket,
    error::{Error, IoContext, Result},
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{collections::HashMap, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{Error as WsError, Message},
};

type Incoming = Option<std::result::Result<Message, WsError>>;
struct Request {
    lease: u64,
    client_id: Value,
    method: String,
}
struct Client {
    socket: WebSocketStream<UnixStream>,
    lease: u64,
}
struct Leases {
    client: Option<Client>,
    next_lease: u64,
    next_id: u64,
    pending: HashMap<u64, Request>,
    sessions: HashMap<String, u64>,
}

/// Serve an already approved connection. EOF revokes it; this never reconnects.
pub(super) async fn run(
    mut browser: BrowserSocket,
    listener: UnixListener,
    control: UnixListener,
) -> Result<()> {
    let mut leases = Leases {
        client: None,
        next_lease: 1,
        next_id: 2, // Startup verification already consumed CDP request id 1.
        pending: HashMap::new(),
        sessions: HashMap::new(),
    };
    loop {
        tokio::select! {
            incoming = control.accept() => {
                let (mut stream, _) = incoming.at("helper control")?;
                let mut message = [0; 5];
                let received = tokio::time::timeout(Duration::from_secs(1), stream.read_exact(&mut message)).await;
                if received.is_ok_and(|value| value.is_ok()) && &message == b"stop\n" {
                    // Drop the transport, never send Browser.close.
                    drop(browser);
                    drop(leases);
                    stream.write_all(&[1]).await.at("disconnect acknowledgement")?;
                    return Ok(());
                }
            }
            incoming = listener.accept(), if leases.client.is_none() => {
                let (stream, _) = incoming.at("helper command")?;
                if let Ok(Ok(socket)) = tokio::time::timeout(Duration::from_secs(2), tokio_tungstenite::accept_async(stream)).await {
                    leases.client = Some(Client { socket, lease: leases.next_lease });
                    leases.next_lease += 1;
                }
            }
            incoming = async { leases.client.as_mut().expect("guarded client").socket.next().await }, if leases.client.is_some() => {
                leases.receive_client(&mut browser, incoming).await?;
            }
            incoming = browser.next() => {
                if !leases.receive_browser(&mut browser, incoming).await? { return Ok(()) }
            }
        }
    }
}

impl Leases {
    async fn receive_client(
        &mut self,
        browser: &mut BrowserSocket,
        incoming: Incoming,
    ) -> Result<()> {
        match incoming {
            Some(Ok(Message::Text(text))) => {
                let mut value: Value = serde_json::from_str(&text)?;
                let method = value["method"]
                    .as_str()
                    .ok_or_else(|| {
                        Error::InvalidArgument("invalid helper protocol request".to_owned())
                    })?
                    .to_owned();
                if self.pending.len() >= 128 {
                    return Err(Error::BrowserBusy);
                }
                let id = self.next_id;
                self.next_id += 1;
                self.pending.insert(
                    id,
                    Request {
                        lease: self.client.as_ref().expect("active client").lease,
                        client_id: value["id"].clone(),
                        method,
                    },
                );
                value["id"] = json!(id);
                send(browser, value).await?;
            }
            Some(Ok(Message::Ping(_))) => {
                self.client
                    .as_mut()
                    .expect("active client")
                    .socket
                    .flush()
                    .await?;
            }
            Some(Ok(Message::Pong(_) | Message::Frame(_) | Message::Binary(_))) => {}
            _ => self.release(browser).await?,
        }
        Ok(())
    }

    async fn release(&mut self, browser: &mut BrowserSocket) -> Result<()> {
        let Some(ended) = self.client.take() else {
            return Ok(());
        };
        let owned: Vec<_> = self
            .sessions
            .iter()
            .filter(|(_, owner)| **owner == ended.lease)
            .map(|(id, _)| id.clone())
            .collect();
        for id in owned {
            self.sessions.remove(&id);
            self.detach(browser, &id).await?;
        }
        Ok(())
    }

    async fn receive_browser(
        &mut self,
        browser: &mut BrowserSocket,
        incoming: Incoming,
    ) -> Result<bool> {
        match incoming {
            Some(Ok(Message::Text(text))) => {
                let mut value: Value = serde_json::from_str(&text)?;
                let forward = if let Some(id) = value["id"].as_u64() {
                    self.route_reply(browser, id, &mut value).await?
                } else {
                    value["sessionId"]
                        .as_str()
                        .and_then(|id| self.sessions.get(id))
                        .is_some_and(|owner| {
                            self.client
                                .as_ref()
                                .is_some_and(|client| client.lease == *owner)
                        })
                };
                if forward {
                    let write = self
                        .client
                        .as_mut()
                        .expect("active recipient")
                        .socket
                        .send(Message::Text(value.to_string().into()));
                    if !tokio::time::timeout(Duration::from_secs(2), write)
                        .await
                        .is_ok_and(|result| result.is_ok())
                    {
                        self.release(browser).await?;
                    }
                }
            }
            Some(Ok(Message::Ping(_))) => {
                browser.flush().await?;
            }
            Some(Ok(Message::Pong(_) | Message::Frame(_) | Message::Binary(_))) => {}
            _ => return Ok(false),
        }
        Ok(true)
    }

    async fn route_reply(
        &mut self,
        browser: &mut BrowserSocket,
        id: u64,
        value: &mut Value,
    ) -> Result<bool> {
        let Some(request) = self.pending.remove(&id) else {
            return Ok(false);
        };
        let active = self
            .client
            .as_ref()
            .is_some_and(|client| client.lease == request.lease);
        if request.method == "Target.attachToTarget"
            && let Some(session) = value["result"]["sessionId"].as_str()
        {
            if active {
                self.sessions.insert(session.to_owned(), request.lease);
            } else {
                self.detach(browser, session).await?;
            }
        }
        value["id"] = request.client_id;
        Ok(active)
    }

    async fn detach(&mut self, browser: &mut BrowserSocket, session: &str) -> Result<()> {
        let id = self.next_id;
        self.next_id += 1;
        send(
            browser,
            json!({"id":id,"method":"Target.detachFromTarget","params":{"sessionId":session}}),
        )
        .await
    }
}

async fn send(browser: &mut BrowserSocket, value: Value) -> Result<()> {
    tokio::time::timeout(
        Duration::from_secs(5),
        browser.send(Message::Text(value.to_string().into())),
    )
    .await
    .map_err(|_| super::disconnected())??;
    Ok(())
}
