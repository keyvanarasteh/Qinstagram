use crate::error::{InstagramError, Result};
use crate::transport::client::InstagramHttpClient;
use crate::types::media::{MessageMedia, MediaCandidate};
use crate::types::message::Message;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

impl InstagramHttpClient {
    pub async fn download_media(&self, url: &str, dest_path: &Path) -> Result<PathBuf> {
        let mut res = self.get(url).send().await?;
        
        let mut file = File::create(dest_path).await.map_err(InstagramError::IoError)?;
        while let Some(chunk) = res.chunk().await.map_err(InstagramError::NetworkError)? {
            file.write_all(&chunk).await.map_err(InstagramError::IoError)?;
        }
        
        Ok(dest_path.to_path_buf())
    }

    pub async fn download_media_from_message(&self, message: &Message, dest_path: &Path) -> Result<PathBuf> {
        let media = match message {
            Message::Media(m) => &m.media,
            _ => return Err(InstagramError::NotImplemented("Extracting media from non-Media Message types".into())),
        };
        
        if let Some(best) = get_best_media_url(media) {
            self.download_media(&best.url, dest_path).await
        } else {
            Err(InstagramError::ApiError("No valid media URL found in message".into()))
        }
    }
}

pub fn get_best_media_url(media: &MessageMedia) -> Option<MediaCandidate> {
    if let Some(mut videos) = media.video_versions.clone() {
        videos.sort_by(|a, b| (b.width * b.height).cmp(&(a.width * a.height)));
        if let Some(best) = videos.into_iter().next() {
            return Some(best);
        }
    }
    
    if let Some(mut images) = media.image_versions2.clone().map(|iv| iv.candidates) {
        images.sort_by(|a, b| (b.width * b.height).cmp(&(a.width * a.height)));
        if let Some(best) = images.into_iter().next() {
            return Some(best);
        }
    }
    
    None
}
