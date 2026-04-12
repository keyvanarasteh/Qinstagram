use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub title: String,
    pub users: Vec<User>,
    pub last_message: Option<Message>,
    pub last_activity: DateTime<Utc>,
    pub unread: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub pk: String,
    pub username: String,
    pub full_name: String,
    pub profile_pic_url: Option<String>,
    pub is_verified: bool,
}
