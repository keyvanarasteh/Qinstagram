use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::types::message::{Message, ReactionEvent, SeenEvent};

/// Real-time events broadcasted from Instagram's SkyWalker MQTT directly to UI websockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum InstagramWsEvent {
    NewMessage(Message),
    Reaction(ReactionEvent),
    ReadReceipt(SeenEvent),
    UserTyping { thread_id: String, user_id: String, is_typing: bool },
    ChallengeRequired { url: String },
}

/// A lightweight wrapper holding the channel broadcaster for standard Tokio integration
pub struct WsBroadcaster {
    pub sender: broadcast::Sender<InstagramWsEvent>,
}

impl Default for WsBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl WsBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self { sender }
    }
}

/// The seamless Axum Drop-In handler to match Qicro's standard exactly
#[cfg(feature = "websocket")]
pub async fn qinstagram_ws_handler(
    mut socket: axum::extract::ws::WebSocket,
    mut rx: broadcast::Receiver<InstagramWsEvent>,
) {
    use axum::extract::ws::Message as AxumWsMessage;

    // Stream the broadcasted realtime IG events securely onto the open websocket
    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            if socket.send(AxumWsMessage::Text(json)).await.is_err() {
                // Connection closed or dropped by frontend
                break;
            }
        }
    }
}
