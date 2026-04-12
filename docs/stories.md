# 🎞️ Stories Engine Module

The `stories` module encompasses interactions around temporary reels, lives, and daily account statuses.

## Trays and Extraction
Utilizes `tray.rs` to aggregate global follower status updates, successfully parsing massive arrays of users emitting `StoryReel` responses.

## Mutating State (`user_stories.rs`)
Safely captures and emits seen-receipt payloads tracking exact timestamps alongside device unique UUID bindings via `mark_stories_as_seen`, signaling the API backend gracefully to drop notifications locally and globally.
