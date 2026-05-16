pub mod common;
pub mod instagram;
pub mod tiktok;
pub mod youtube;
pub mod twitter;
pub mod facebook;
pub mod snapchat;

use async_trait::async_trait;
use crate::error::{AppError, AppResult};
use crate::models::{Platform, VideoMetadata};

#[async_trait]
pub trait PlatformAdapter: Send + Sync {
    /// Validate if URL belongs to this platform
    async fn validate_url(&self, url: &str) -> AppResult<bool>;

    /// Fetch video metadata from platform
    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata>;

    /// Get direct download URL for video
    async fn get_download_url(&self, url: &str) -> AppResult<String>;
}

pub fn get_adapter_for_platform(platform: Platform) -> AppResult<Box<dyn PlatformAdapter>> {
    match platform {
        Platform::Instagram => Ok(Box::new(instagram::InstagramAdapter)),
        Platform::TikTok => Ok(Box::new(tiktok::TikTokAdapter)),
        Platform::YouTube => Ok(Box::new(youtube::YouTubeAdapter)),
        Platform::Twitter => Ok(Box::new(twitter::TwitterAdapter)),
        Platform::Facebook => Ok(Box::new(facebook::FacebookAdapter)),
        Platform::Snapchat => Ok(Box::new(snapchat::SnapchatAdapter)),
        Platform::Unknown => Err(AppError::PlatformNotSupported(
            "Unknown platform".to_string(),
        )),
    }
}
