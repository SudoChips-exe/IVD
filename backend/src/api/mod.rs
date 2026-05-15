pub mod common;
pub mod instagram;
pub mod tiktok;
pub mod youtube;
pub mod twitter;
pub mod facebook;
pub mod snapchat;

use async_trait::async_trait;
use crate::error::AppResult;
use crate::models::VideoMetadata;

#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Validate if URL belongs to this platform
    async fn validate_url(&self, url: &str) -> AppResult<bool>;

    /// Fetch video metadata from platform
    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata>;

    /// Get direct download URL for video
    async fn get_download_url(&self, url: &str) -> AppResult<String>;
}
