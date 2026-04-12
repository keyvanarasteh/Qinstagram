use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCandidate {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageVersions {
    pub candidates: Vec<MediaCandidate>,
}

pub type VideoVersion = MediaCandidate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMedia {
    pub id: String,
    pub media_type: u8,
    pub image_versions2: Option<ImageVersions>,
    pub video_versions: Option<Vec<MediaCandidate>>,
    pub original_width: u32,
    pub original_height: u32,
}
