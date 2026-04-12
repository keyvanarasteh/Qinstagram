# Qinstagram Documentation Index

Welcome to the comprehensive documentation for the **Qinstagram** Rust crate.

## 📦 Core Modules

- [**Authentication & Session (`auth`)**](auth.md)
  - Security Checkpoints, 2FA, Multi-user Session Injection
- [**Direct Ecosystem (`direct`)**](direct.md)
  - Message parsing, Event Broadcasting, Media Upload logic
- [**Storytelling (`stories`)**](stories.md)
  - Extracting Reels Trays and marking media as seen
- [**Feed API (`feed`)**](feed.md)
  - Handling user timelines and news inboxes

## ⚙️ Architecture Information

Qinstagram utilizes `tokio` for zero-cost async handling and perfectly mimics the behavior of Android Instagram APIs, resolving challenges using safe, robust, unwrap-free Rust logic.
