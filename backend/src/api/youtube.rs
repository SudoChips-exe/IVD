use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;

pub struct YouTubeAdapter;

#[async_trait]
impl PlatformAdapter for YouTubeAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("youtube.com") || url_lower.contains("youtu.be"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        // TODO: Implement YouTube Data API v3 integration
        // Note: YouTube API doesn't provide direct download access
        // Will need to use yt-dlp library or similar

        Err(AppError::InternalServerError(
            "YouTube integration not yet implemented".to_string(),
        ))
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        // TODO: Extract video stream URL
        Err(AppError::InternalServerError(
            "YouTube integration not yet implemented".to_string(),
        ))
    }
}
