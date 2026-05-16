use async_trait::async_trait;
use crate::api::common;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;

pub struct TwitterAdapter;

#[async_trait]
impl PlatformAdapter for TwitterAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("twitter.com") || url_lower.contains("x.com"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        let html = common::fetch_html(url).await?;
        let video_url = common::extract_meta_content(&html, "og:video:secure_url")
            .or_else(|| common::extract_meta_content(&html, "og:video"))
            .or_else(|| common::extract_meta_content(&html, "twitter:player:stream"))
            .ok_or_else(|| AppError::VideoNotFound("Twitter video URL not found".to_string()))?;

        Ok(VideoMetadata {
            title: common::extract_meta_content(&html, "og:title").unwrap_or_else(|| "Twitter Video".to_string()),
            duration_seconds: 0,
            author: common::extract_meta_content(&html, "og:site_name").unwrap_or_else(|| "Twitter".to_string()),
            video_url,
            audio_url: None,
            thumbnail_url: common::extract_meta_content(&html, "og:image").unwrap_or_default(),
            original_platform: "twitter".to_string(),
            file_size_bytes: None,
        })
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        let metadata = self.fetch_metadata(url).await?;
        Ok(metadata.video_url)
    }
}
