use async_trait::async_trait;
use crate::api::common;
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
        let html = common::fetch_html(url).await?;
        let video_url = common::extract_meta_content(&html, "og:video:secure_url")
            .or_else(|| common::extract_meta_content(&html, "og:video"))
            .ok_or_else(|| AppError::VideoNotFound("YouTube video URL not found".to_string()))?;

        Ok(VideoMetadata {
            title: common::extract_meta_content(&html, "og:title").unwrap_or_else(|| "YouTube Video".to_string()),
            duration_seconds: 0,
            author: common::extract_meta_content(&html, "og:site_name").unwrap_or_else(|| "YouTube".to_string()),
            video_url,
            audio_url: None,
            thumbnail_url: common::extract_meta_content(&html, "og:image").unwrap_or_default(),
            original_platform: "youtube".to_string(),
            file_size_bytes: None,
        })
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        let metadata = self.fetch_metadata(url).await?;
        Ok(metadata.video_url)
    }
}
