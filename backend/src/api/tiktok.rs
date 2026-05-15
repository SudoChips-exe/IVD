use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;

pub struct TikTokAdapter;

#[async_trait]
impl PlatformAdapter for TikTokAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("tiktok.com")
            || url_lower.contains("vm.tiktok.com")
            || url_lower.contains("vt.tiktok.com"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        // TODO: Implement TikTok API integration
        // This will use either:
        // 1. Official TikTok API (limited video access)
        // 2. TikTok-Api-Sharp library
        // 3. Custom extraction

        Err(AppError::InternalServerError(
            "TikTok integration not yet implemented".to_string(),
        ))
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        // TODO: Extract direct video URL with audio
        Err(AppError::InternalServerError(
            "TikTok integration not yet implemented".to_string(),
        ))
    }
}
