use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;

pub struct SnapchatAdapter;

#[async_trait]
impl PlatformAdapter for SnapchatAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("snapchat.com") || url_lower.contains("snap.com"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        // TODO: Implement Snapchat integration
        // Note: Snapchat has very limited public API access
        // May require user credentials or custom extraction

        Err(AppError::InternalServerError(
            "Snapchat integration not yet implemented".to_string(),
        ))
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        // TODO: Extract video stream URL
        Err(AppError::InternalServerError(
            "Snapchat integration not yet implemented".to_string(),
        ))
    }
}
