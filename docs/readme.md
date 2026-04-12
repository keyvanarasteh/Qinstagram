# Qinstagram Documentation Index

Welcome to the comprehensive documentation for the **Qinstagram** Rust crate.

## 📦 Core Modules

| Document | Description |
|---|---|
| [Authentication & Session](auth.md) | Security Checkpoints, 2FA, Multi-user Session Injection |
| [Direct Messaging](direct.md)      | Send messages, fetch inboxes, upload media (Image/Video targeting thread), fuzzy search |
| [Stories & Reels](stories.md)      | Fetch timeline trays, load specific user histories, and mutate read state |
| [Feeds & Discovery](feed.md)       | Browse main timelines and news alerts |

### Advanced Architectural Integrations
| Document | Description |
|---|---|
| [GraphQL Coverage](graphql.md) | Standard SDK manual for mounting schemas dropping seamlessly into the Qicro interface |
| [WebSocket Stream](websocket.md) | Manual for pushing live telemetry onto single websocket proxies preventing REST polling limits |

## ⚙️ Architecture Information

Qinstagram utilizes `tokio` for zero-cost async handling and perfectly mimics the behavior of Android Instagram APIs, resolving challenges using safe, robust, unwrap-free Rust logic.
