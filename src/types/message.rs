use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::media::{MessageMedia};
use super::post::Post;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub emoji: String,
    pub sender_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionEvent {
    pub thread_id: String,
    pub item_id: String,
    pub user_id: String,
    pub emoji: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeenEvent {
    pub thread_id: String,
    pub user_id: String,
    pub item_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepliedToMessage {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub text: Option<String>,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "item_type")]
pub enum Message {
    #[serde(rename = "text")]
    Text(TextMessage),
    #[serde(rename = "media")]
    Media(MediaMessage),
    #[serde(rename = "link")]
    Link(LinkMessage),
    #[serde(rename = "placeholder")]
    Placeholder(PlaceholderMessage),
    #[serde(rename = "media_share")]
    MediaShare(MediaShareMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseMessage {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub username: String,
    pub is_outgoing: bool,
    pub thread_id: String,
    pub reactions: Option<Vec<Reaction>>,
    pub replied_to: Option<RepliedToMessage>,
    pub item_id: Option<String>,
    pub client_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    #[serde(flatten)]
    pub base: BaseMessage,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMessage {
    #[serde(flatten)]
    pub base: BaseMessage,
    pub media: MessageMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkMessage {
    #[serde(flatten)]
    pub base: BaseMessage,
    pub link: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderMessage {
    #[serde(flatten)]
    pub base: BaseMessage,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaShareMessage {
    #[serde(flatten)]
    pub base: BaseMessage,
    pub media_share_post: Post,
    pub media_share_index: Option<u32>,
}
