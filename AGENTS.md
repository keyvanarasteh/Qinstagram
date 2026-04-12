# AGENTS.md

This file provides context for AI coding assistants (Cursor, GitHub Copilot, Claude Code, Gemini, etc.) working with the Qinstagram payload module.

## Project Overview

**Qinstagram** is a Rust library component designed for high-performance interaction with Instagram's Private API, porting capabilities from former Node/TypeScript libraries to robust native code.

- **Repository**: https://github.com/keyvanarasteh/Qinstagram
- **Crate**: https://crates.io/crates/qinstagram
- **License**: MIT OR Apache-2.0

## Repository Structure

```
Qinstagram/
├── src/
│   ├── lib.rs                  # Crate root, modules, public re-exports
│   ├── auth/                   # Checkpoints, login, SessionManager, 2FA
│   ├── direct/                 # DMs, threads, message broadcasting, inbox
│   ├── feed/                   # Timeline, posts
│   ├── stories/                # User stories, reels tray
│   ├── realtime/               # MQTT connection, skywalker, graphql (feature gated)
├── examples/                   # Runnable examples (e.g., login.rs, inbox.rs)
├── tests/                      # Integration test suites for each module
```

## Development Commands

```bash
cargo build --all-features
cargo clippy --workspace --all-targets -- -D warnings
cargo test --all-features
cargo run --example login
```

## Key Patterns
- Feature-gating: Always `#![cfg(feature="...")]` wrap your modules (e.g., `realtime`).
- Error Handling: Return `InstagramError` variants using `thiserror`. Never `unwrap()` in `src/`!
- Modularity: Add new facades via `src/client.rs`.
