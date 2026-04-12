# 💬 Direct Ecosystem Module

The `direct` module orchestrates all messaging interactions within the Instagram DM ecosystem.

## Sub-Modules
### 1. `inbox.rs`
Handles pagination of user threads. Implements highly useful data extraction pipelines such as `search_threads_by_title` utilizing Jaro-Winkler fuzzy metrics alongside targeted identity searching via `search_thread_by_username`.

### 2. `broadcast.rs`
Manages the HTTP distribution of communication data logic. Supports:
- Sending robust plaintext responses and link formatting
- Broadcast interactions (`send_reaction` emojis) over HTTP
- Message modification capabilities (`unsend_message`)
- Safe seen-receipt handling (`mark_item_as_seen`) falling back against MQTT architecture protocols.

### 3. `parser.rs`
Given the polymorphic JSON data generated via IG Direct, `parse_message_item` safely translates arbitrary dynamically-structured elements into the fully strong-typed `Message` enumeration mapping system.

## File Handling (`upload.rs`)
Manages deep interaction with Facebook's custom `rupload` API layer, chunking direct photo and mp4 video streams through their independent configuration waterfall API pipeline perfectly mapping HTTP boundaries.
