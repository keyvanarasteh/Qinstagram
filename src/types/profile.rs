use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct ProfileInfo {
    pub pk: String,
    pub username: String,
    pub full_name: String,
    pub profile_pic_url: Option<String>,
    pub is_verified: bool,
    pub is_private: bool,
    pub biography: String,
    pub follower_count: u64,
    pub following_count: u64,
    pub media_count: u64,
    pub external_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct AuthState {
    pub is_logged_in: bool,
    pub username: Option<String>,
    pub user_id: Option<String>,
    pub loading: bool,
    pub error: Option<String>,
}
