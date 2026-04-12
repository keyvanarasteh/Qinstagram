# 🛰️ WebSocket & MQTT Syndication

> ⚠️ Requires `#![cfg(feature = "websocket")]`

Instagram natively delivers live inbox events via **MQTT (Facebook Skywalker)**. Relying on HTTP long-polling (`client.get_threads()`) rapidly incurs API rate-limits and shadowbans.

We provide a seamless out-of-the-box translation layer that binds Instagram's raw binary MQTT feeds securely to `axum` WebSockets utilizing `tokio::sync::broadcast`!

## Architectural Drop-in

```rust
use qinstagram::realtime::mqtt::RealtimeClient;
use qinstagram::ws::{WsBroadcaster, qinstagram_ws_handler};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 1. Create the Toko Multi-Producer Multi-Consumer Broadcaster
    let broadcaster = WsBroadcaster::new();

    // 2. Attach it to your Instagram Realtime MQTT Client Loop
    let mut mqtt_core = RealtimeClient::new()
        .with_broadcaster(broadcaster.sender.clone());

    // 3. Start Skywalker in the background bridging Facebook to Rust
    tokio::spawn(async move {
        mqtt_core.connect().await;
    });

    // 4. Expose the native Websocket directly onto your application router!
    let tx = broadcaster.sender.clone();
    let app = Router::new().route("/ws/instagram", get(move |ws| async move {
        qinstagram_ws_handler(ws, tx.subscribe()).await
    }));

    // Start server...
}
```

### Live `InstagramWsEvent` Telemetry

The JSON payload sent down the socket utilizes type-tagging `#[serde(tag = "type", content = "payload")]` perfectly mirroring Qicro frontend models.

- `NewMessage(Message)` -> Stream incoming text, media, and thread links
- `Reaction(ReactionEvent)` -> Realtime emoji popup alerts
- `ReadReceipt(SeenEvent)` -> See exactly when remote user reads your packet
- `UserTyping` -> "User X is typing..." indicators
- `ChallengeRequired` -> Auth 2FA live rescue flows without reloading UI
