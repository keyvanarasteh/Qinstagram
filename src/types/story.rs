use serde::{Deserialize, Serialize};

use super::media::{ImageVersions, VideoVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryUser {
    pub pk: u64,
    pub username: String,
    pub profile_pic_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReelMentionUser {
    pub pk: u64,
    pub username: String,
    pub full_name: String,
    pub profile_pic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReelMention {
    pub user: ReelMentionUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct StoryReel {
    pub user: StoryUser,
    pub stories: Vec<Story>,
}
