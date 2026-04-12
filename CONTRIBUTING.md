# Contributing to Qinstagram

Thank you for your interest in contributing to Qinstagram! As an enterprise-grade private API architecting tool, we rely on community insights to stay ahead of platform mutations.

## 🚀 Getting Started

1. **Fork the Repository:** Create your own fork and clone it to your local workspace.
2. **Setup Rust:** Ensure you have the latest stable Rust toolchain via `rustup`.
3. **Compile:** Run `cargo build --all-features` to compile all integrations (including our GraphQL and WebSocket submodules).

## 🛠️ Code Structure

- **`src/auth`:** Checkpoint & session persistence logic.
- **`src/direct`:** Core unified messaging & uploading capabilities.
- **`src/graphql.rs`:** Explicit Native declarative endpoint mappings matching `async-graphql`.
- **`src/ws/mod.rs`:** Advanced WebSocket streaming bridging rumqttc binaries.

## 🧪 Testing

Given we are interacting with remote platform APIs, please ensure you mock all responses locally inside `tests/` before PR-ing.

```bash
cargo test --all-features
```

## 📜 Pull Request Guidelines

- Ensure `cargo fmt` and `cargo clippy --all-features -- -D warnings` exit flawlessly.
- Branch off from `main` and submit your Pull Request to the `main` branch.
- Detail exactly what API signatures you isolated or what JSON mutations triggered the change.
