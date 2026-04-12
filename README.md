<p align="center">
  <h1 align="center">📸 Qinstagram</h1>
  <p align="center">
    <strong>Enterprise Private API Architecture & Real-Time Engagement Kit</strong>
  </p>
  <p align="center">
    High-performance Rust toolkit for advanced Instagram Direct messaging, automated feeds, and real-time storytelling
  </p>
</p>

<p align="center">
  <a href="https://crates.io/crates/qinstagram"><img src="https://img.shields.io/crates/v/qinstagram?style=for-the-badge&logo=rust&logoColor=white&color=orange" alt="crates.io"></a>
  <a href="https://docs.rs/qinstagram"><img src="https://img.shields.io/docsrs/qinstagram?style=for-the-badge&logo=docs.rs&logoColor=white" alt="docs.rs"></a>
  <a href="https://github.com/keyvanarasteh/Qinstagram/actions"><img src="https://img.shields.io/github/actions/workflow/status/keyvanarasteh/Qinstagram/ci.yml?style=for-the-badge&logo=github&label=CI" alt="CI"></a>
  <a href="https://github.com/keyvanarasteh/Qinstagram/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/qinstagram?style=for-the-badge" alt="License"></a>
</p>

<p align="center">
  <a href="#"><img src="https://img.shields.io/badge/async-tokio-purple?style=flat-square&logo=tokio" alt="Tokio"></a>
  <a href="#"><img src="https://img.shields.io/badge/http-reqwest-blue?style=flat-square" alt="Reqwest"></a>
  <a href="#"><img src="https://img.shields.io/badge/mqtt-rumqttc-red?style=flat-square" alt="MQTT"></a>
  <a href="#"><img src="https://img.shields.io/badge/serialization-serde-orange?style=flat-square" alt="Serde"></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-linux-lightgrey?style=flat-square&logo=linux" alt="Linux"></a>
</p>

---

## 🚀 Quick Start

```bash
cargo add qinstagram
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
qinstagram = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use qinstagram::client::InstagramClient;
use qinstagram::config::LoginConfig;

#[tokio::main]
async fn main() {
    let mut client = InstagramClient::builder()
        .credentials("username", "password")
        .build()
        .unwrap();

    // Authenticate and establish secure session (supports 2FA & Checkpoints)
    client.login().await.unwrap();

    // Directly access messaging infrastructure
    let threads = client.get_threads(None).await.unwrap();
    println!("Fetched {} direct threads", threads.threads.len());
}
```

> **Selective features**: `cargo add qinstagram --no-default-features --features realtime,media`

---


## ✨ Features

- **4 distinct workflow modules** cleanly organized reflecting Instagram's domain architecture.
- **Panic-free Library** — robust error handling wrapping and propagating `InstagramError` instances.
- **Complex Payload Parser Ecosystem** — rigorously handles dynamic and polymorphic responses.
- **Two-Factor and Challenge Flows** — seamlessly navigate verification checkpoint gates.
- **MQTT Event Capabilities** — unified fallback messaging and real-time syncing.
- **Multi-user Engine** — robust session token memory and cross-context swapping.
- **Ruploado Media Configuration** — highly complex custom media routing built manually into Rust.
- **Reverse-engineered Types** — modeled dynamically directly off typescript AST ports for strict parity.

---

## 📦 Module Overview

### 🔐 Authentication & Session

| Module               | Description                                                                                                               | Docs                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| **auth**             | Stateful Password login, 2FA challenge responses, Session lifecycle & persistent multi-user credentials                    | [📖](docs/auth.md)      |
| **transport**        | Payload signing, deterministic device ID generation, request header synthesis, and Android emulation                       | [📖](docs/auth.md)       |

### 💬 Direct Communication

| Module                       | Description                                                           | Docs                                   |
| ---------------------------- | --------------------------------------------------------------------- | -------------------------------------- |
| **unified_direct**           | Direct message fetching, fuzzy inbox search, message parser engine, un-sending   | [📖](docs/direct.md)              |
| **media_upload**             | Two-step direct media configuration and rupload data injection        | [📖](docs/direct.md) |
| **broadcast**                | Liking, reacting, responding with URL parsing and state tracking        | [📖](docs/direct.md) |

### 🎞️ Experiences

| Module                   | Description                                                                                 | Docs                               |
| ------------------------ | ------------------------------------------------------------------------------------------- | ---------------------------------- |
| **stories**              | Story and reels gathering, tray parsing, real-time media seen tracking capabilities         | [📖](docs/stories.md)    |
| **feed**                 | News inbox analysis, timeline extraction and pagination        | [📖](docs/feed.md)   |
| **realtime**             | MQTT Skywalker integration behind `--features realtime`                                    | [📖](docs/realtime.md)        |

> 📚 **Full documentation index:** [docs/readme.md](docs/readme.md)

---

## 🚀 Usage

### Session Injection & Media Tracking

```rust
use qinstagram::client::InstagramClient;

#[tokio::main]
async fn main() {
    let client = InstagramClient::from_session("username").await.unwrap();

    // Exfiltrate media targeting
    let media_candidates = client.get_reels_tray().await.unwrap();
    
    // Broadcast Seen State back to network
    client.mark_stories_as_seen(&media_candidates[0].stories).await.unwrap();
    
    // Send Reactions
    let target_thread = client.search_thread_by_username("friend_account").await.unwrap();
    client.send_reaction(&target_thread.unwrap().thread_id, "12345_67", "🔥").await.unwrap();
}
```

---

## ⚙️ Feature Flags

Include only what you need:

```toml
[dependencies]
qinstagram = { version = "0.1.0", features = ["realtime", "tests-mock"] }
```

<details>
<summary><strong>All feature flags</strong></summary>

```toml
# Subsystem Engines
realtime = []
media = []

# Development Mocks
tests-mock = []
```

</details>

---

## 🏗️ Build

```bash
# Core framework
cargo build

# All experimental modules
cargo build --all-features

# Release compilation
cargo build --all-features --release

# Run rigorous unit tests and offline mock parsers
cargo test --all-features
```

---

## 📁 Project Structure

```
Qinstagram/
├── Cargo.toml                        # Dependencies & architectural flags
├── README.md                         # This file
├── src/
│   ├── lib.rs                        # API facade and Module exports
│   │
│   │── auth/                         # Sessions, Challenges & Verification
│   │── direct/                       # Thread logic, Search, Broadcast, Parser
│   │── feed/                         # Core API Timelines
│   │── media/                        # Downloads & Complex Upload Protocols
│   │── realtime/                     # Under heavy development MQTT
│   │── stories/                      # Trays & state persistence
│   │── transport/                    # Emulation context and networking
│   └── types/                        # Struct parity wrappers
│
├── docs/                             # Component Reference Documentation 
│   ├── readme.md                     # Documentation index
│   └── [module_name].md
│
├── examples/                         # Real-world usage scenarios
└── tests/                            # Offline JSON structural integrity testing
```

---

## 📊 Stats

| Metric                 | Value |
| ---------------------- | ----- |
| Total modules          | 9    |
| Extracted Data Types   | ~60    |
| Real-time MQTT endpoints| 2    |
| HTTP Message Builders  | 14    |
| Supported Media Upload | Photo + Video + Link |

---

## 👤 Author

<table>
  <tr>
    <td align="center">
      <a href="https://github.com/keyvanarasteh">
        <img src="https://github.com/keyvanarasteh.png" width="80" height="80" style="border-radius:50%" alt="Keyvan Arasteh"><br>
        <sub><b>Keyvan Arasteh</b></sub><br>
        <sub>@keyvanarasteh</sub>
      </a>
    </td>
  </tr>
</table>

---

<p align="center">
  <sub>Built with 🦀 Rust — İstinye University</sub>
</p>
