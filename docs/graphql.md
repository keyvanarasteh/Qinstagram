# 🔌 GraphQL API Bindings

> ⚠️ Requires `#![cfg(feature = "graphql")]`

`Qinstagram` natively ships with 100% GraphQL schema coverage built on top of `async-graphql` version 7.0. It exposes all core Instagram structures as `SimpleObject` representations and provides two massive pre-built endpoint routers.

## 🧠 Qicro `ai-core` Standard
This implementation was rigorously developed to drop directly into the **Qicro AI Ecosystem** as a standalone remote data source.

### Architecture Injection
You do not need to rewrite mapping definitions. Simply inject an instantiated `Arc<InstagramClient>` into your master Context container, and merge our query/mutation schema roots:

```rust
use async_graphql::{Schema, EmptySubscription};
use qinstagram::graphql::{QinstagramQuery, QinstagramMutation};

#[tokio::main]
async fn main() {
    let client = create_and_login_your_client().await;
    
    let schema = Schema::build(
        QinstagramQuery::default(),
        QinstagramMutation::default(),
        EmptySubscription,
    )
    .data(std::sync::Arc::new(client))
    .finish();
}
```

### Supported Mapping Parity (100% Coverage)

**Data Objects Derived**: `User`, `Thread`, `Story`, `StoryReel`, `ProfileInfo`, `AuthState`, `LoginResult`, `InboxResult`, `NewsInbox`, `MessagesResult`, and all polymorphic `Message` variants.

**Query Resolvers**: 
- `current_user`, `direct_threads`, `thread_by_username`, `reels_tray`
- `news_inbox`, `messages`, `timeline_feed`, `stories_for_user`
- `search_users`, `search_user_exact`, `search_threads_by_title`
- `user_info_by_pk`, `user_profile_by_username`

**Mutation Resolvers**:
- `login`, `two_factor_login`, `start_challenge`, `send_challenge_code`
- `switch_user`, `logout`, `cleanup_sessions`
- `send_message`, `send_reply`, `send_reaction`, `unsend_message`
- `mark_item_seen`, `mark_thread_seen`, `ensure_thread`
- `send_photo`, `send_video`
