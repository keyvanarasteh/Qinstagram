use serde::{Deserialize, Serialize};

use super::media::{ImageVersions, VideoVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostUser {
    pub pk: u64,
    pub username: String,
    pub profile_pic_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caption {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselItem {
    pub id: String,
    pub media_type: u8,
    pub image_versions2: Option<ImageVersions>,
    pub video_versions: Option<Vec<VideoVersion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub user: PostUser,
    pub caption: Option<Caption>,
    pub image_versions2: Option<ImageVersions>,
    pub like_count: u64,
    pub comment_count: u64,
    pub taken_at: i64,
    pub media_type: u8,
    pub video_versions: Option<Vec<VideoVersion>>,
    pub carousel_media_count: Option<u32>,
    pub carousel_media: Option<Vec<CarouselItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedInstance {
    pub posts: Vec<Post>,
}
