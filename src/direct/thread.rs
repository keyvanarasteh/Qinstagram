use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct MessagesResult {
    pub messages: Vec<Message>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl MessagesResult {
    /// Keep the latest `limit` messages while preserving chronological order
    /// (oldest → newest).
    ///
    /// Mirrors the instagram-cli fix for `read --limit`: callers that want the
    /// most recent N messages should not take the *first* N of a page (which
    /// are the oldest). When the page is truncated, `has_more` is set and
    /// `next_cursor` points at the oldest displayed message so older history
    /// can be fetched without skipping.
    pub fn take_latest(mut self, limit: usize) -> Self {
        if limit == 0 {
            self.messages.clear();
            self.has_more = false;
            self.next_cursor = None;
            return self;
        }

        if self.messages.len() > limit {
            let start = self.messages.len() - limit;
            self.messages = self.messages.split_off(start);
            self.has_more = true;
            self.next_cursor = self
                .messages
                .first()
                .and_then(message_pagination_id)
                .map(|s| s.to_string());
        }

        self
    }

    /// Item id of the newest message (last in chronological order).
    /// Use this for `mark_thread_as_seen` so the thread is fully marked read.
    pub fn latest_item_id(&self) -> Option<&str> {
        self.messages.last().and_then(message_pagination_id)
    }

    /// Item id of the oldest message (first in chronological order).
    pub fn oldest_item_id(&self) -> Option<&str> {
        self.messages.first().and_then(message_pagination_id)
    }
}

fn message_pagination_id(msg: &Message) -> Option<&str> {
    let base = match msg {
        Message::Text(m) => &m.base,
        Message::Media(m) => &m.base,
        Message::Link(m) => &m.base,
        Message::Placeholder(m) => &m.base,
        Message::MediaShare(m) => &m.base,
    };
    base.item_id
        .as_deref()
        .filter(|s| !s.is_empty())
        .or(Some(base.id.as_str()))
}

impl InstagramHttpClient {
    pub async fn get_messages(&self, thread_id: &str, cursor: Option<&str>) -> Result<MessagesResult> {
        let mut url = format!("{}/api/v1/direct_v2/threads/{}/", crate::constants::HOST, thread_id);
        if let Some(c) = cursor {
            url.push_str(&format!("?cursor={}", url::form_urlencoded::byte_serialize(c.as_bytes()).collect::<String>()));
        }

        let res = self.get(&url)
            .send()
            .await?;
            
        let text = res.text().await.map_err(InstagramError::NetworkError)?;
        let json_res: serde_json::Value = serde_json::from_str(&text).map_err(InstagramError::SerdeError)?;
        
        let mut messages = Vec::new();
        let current_user_id = self.get_cookie_value("ds_user_id").unwrap_or_default();

        if let Some(thread) = json_res.get("thread") {
            if let Some(items_arr) = thread.get("items").and_then(|i| i.as_array()) {
                for item_val in items_arr {
                    if let Some(msg) = crate::direct::parser::parse_message_item(item_val, thread_id, &current_user_id) {
                        messages.push(msg);
                    }
                }
            }
        }
        
        // Instagram returns newest-first; reverse to chronological (oldest → newest).
        messages.reverse();
        
        let has_more = json_res.get("thread").and_then(|i| i.get("has_older")).and_then(|h| h.as_bool()).unwrap_or(false);
        let next_cursor = json_res.get("thread").and_then(|i| i.get("oldest_cursor")).and_then(|c| c.as_str()).map(|s| s.to_string());
        
        Ok(MessagesResult {
            messages,
            has_more,
            next_cursor,
        })
    }

    /// Fetch messages and return only the latest `limit` (chronological order preserved).
    /// Prefer this over slicing the front of `get_messages` when implementing "last N" UX.
    pub async fn get_latest_messages(
        &self,
        thread_id: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<MessagesResult> {
        let result = self.get_messages(thread_id, cursor).await?;
        Ok(result.take_latest(limit))
    }
}
