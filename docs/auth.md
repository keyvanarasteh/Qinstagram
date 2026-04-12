# 🔐 Authentication Module

The `auth` module provides the foundational capabilities for securely logging into Instagram and storing credentials safely.

## Key Features
- **Dynamic Device Simulation:** Automatically mimics Samsung/Xiaomi devices with randomized identifiers to avoid rapid flag responses.
- **Two-Factor Authentication (2FA):** Handles endpoints requesting offline token backup authentication natively.
- **Security Challenges & Checkpoints:** Leverages `start_challenge` and `send_challenge_code` verification flows to break out of API sandboxing lockouts.
- **Session Swapping:** Safely persists and loads cookie stores (`cookie_store::CookieStore`) enabling `InstagramClient` multi-user concurrency tracking.

## Usage
```rust
use qinstagram::client::InstagramClient;

// Automatically handles device serialization and caching
client.login().await?;
client.switch_user("another_user").await?;
```
