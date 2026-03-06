use anyhow::{bail, Context, Result};
use futures::{SinkExt, StreamExt};
use nest_protocol::entity::{StreamGroupId, StreamId};
use nest_protocol::messages::{ClientMessage, CrdtOp, ServerMessage, StreamPosition};
use tokio_tungstenite::tungstenite::Message;

/// WebSocket sync client for real-time synchronization.
pub(crate) struct SyncClient {
    pub url: String,
    pub token: Option<String>,
    #[allow(dead_code)]
    pub node_id: u128,
}

impl SyncClient {
    pub fn new(url: &str, node_id: u128) -> Self {
        SyncClient {
            url: url.to_string(),
            token: None,
            node_id,
        }
    }

    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.token = token;
        self
    }

    /// Perform a full sync: send local ops, receive remote ops.
    pub async fn sync(
        &self,
        group_id: &StreamGroupId,
        local_streams: Vec<(StreamId, Vec<CrdtOp>)>,
        known_positions: Vec<StreamPosition>,
    ) -> Result<Vec<(StreamId, Vec<CrdtOp>)>> {
        let ws_url = self.url.replace("http://", "ws://").replace("https://", "wss://");
        let (ws_stream, _) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .context("Failed to connect to sync server")?;

        let (mut writer, mut reader) = ws_stream.split();

        // Authenticate if token provided
        if let Some(ref token) = self.token {
            let auth_msg = ClientMessage::Auth {
                token: token.clone(),
            };
            let json = serde_json::to_string(&auth_msg)?;
            writer.send(Message::Text(json.into())).await?;

            // Wait for AuthOk
            if let Some(Ok(Message::Text(text))) = reader.next().await {
                let msg: ServerMessage = serde_json::from_str(&text)?;
                match msg {
                    ServerMessage::AuthOk { .. } => {}
                    ServerMessage::Error { message, .. } => {
                        bail!("Auth failed: {}", message);
                    }
                    _ => {}
                }
            }
        }

        // Request remote ops we don't have
        let request = ClientMessage::RequestOps {
            group: group_id.clone(),
            known_positions,
        };
        let json = serde_json::to_string(&request)?;
        writer.send(Message::Text(json.into())).await?;

        // Send local ops
        for (stream_id, ops) in &local_streams {
            if ops.is_empty() {
                continue;
            }
            let msg = ClientMessage::StreamOps {
                stream_id: stream_id.clone(),
                ops: ops.clone(),
            };
            let json = serde_json::to_string(&msg)?;
            writer.send(Message::Text(json.into())).await?;
        }

        // Collect remote ops with a timeout
        let mut remote_ops = Vec::new();
        let timeout = tokio::time::Duration::from_secs(5);

        loop {
            match tokio::time::timeout(timeout, reader.next()).await {
                Ok(Some(Ok(Message::Text(text)))) => {
                    let text_str: &str = &text;
                    if let Ok(msg) = serde_json::from_str::<ServerMessage>(text_str) {
                        match msg {
                            ServerMessage::StreamOps { stream_id, ops } => {
                                remote_ops.push((stream_id, ops));
                            }
                            ServerMessage::Error { message, .. } => {
                                eprintln!("Server error: {}", message);
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Some(Ok(Message::Close(_)))) | Ok(None) => break,
                Ok(Some(Err(e))) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
                Ok(Some(Ok(_))) => {} // ignore other message types
                Err(_) => break,      // timeout, done receiving
            }
        }

        // Close connection
        let _ = writer.send(Message::Close(None)).await;

        Ok(remote_ops)
    }
}
