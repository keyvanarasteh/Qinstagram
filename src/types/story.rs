use serde::{Deserialize, Serialize};

use super::media::{ImageVersions, VideoVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct StoryUser {
    pub pk: u64,
    pub username: String,
    /// Display name from Instagram (`full_name` on user objects).
    #[serde(default)]
    pub full_name: Option<String>,
    pub profile_pic_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct ReelMentionUser {
    pub pk: u64,
    pub username: String,
    pub full_name: String,
    pub profile_pic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct ReelMention {
    pub user: ReelMentionUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Story {
    pub id: String,
    pub user: StoryUser,
    pub reel_mentions: Option<Vec<ReelMention>>,
    pub image_versions2: Option<ImageVersions>,
    pub video_versions: Option<Vec<VideoVersion>>,
    pub taken_at: i64,
    pub media_type: u8, // 1 = image, 2 = video
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct StoryReel {
    pub user: StoryUser,
    /// Story media for this reel. Instagram tray payloads use `items`;
    /// filled reels and some clients use `stories`.
    #[serde(default, alias = "items")]
    pub stories: Vec<Story>,
}
