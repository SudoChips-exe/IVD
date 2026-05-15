use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;

pub struct FacebookAdapter;

#[async_trait]
impl PlatformAdapter for FacebookAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("facebook.com") || url_lower.contains("fb.watch"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        // TODO: Implement Facebook Graph API integration
        // Requires app token and video lookup

        Err(AppError::InternalServerError(
            "Facebook integration not yet implemented".to_string(),
        ))
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        // TODO: Extract video stream URL
        Err(AppError::InternalServerError(
            "Facebook integration not yet implemented".to_string(),
        ))
    }
}
