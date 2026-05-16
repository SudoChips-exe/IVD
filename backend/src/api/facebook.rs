use async_trait::async_trait;
use crate::api::common;
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
        let html = common::fetch_html(url).await?;
        let video_url = common::extract_meta_content(&html, "og:video:secure_url")
            .or_else(|| common::extract_meta_content(&html, "og:video"))
            .ok_or_else(|| AppError::VideoNotFound("Facebook video URL not found".to_string()))?;

        Ok(VideoMetadata {
            title: common::extract_meta_content(&html, "og:title").unwrap_or_else(|| "Facebook Video".to_string()),
            duration_seconds: common::extract_meta_content(&html, "video:duration")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0),
            author: common::extract_meta_content(&html, "og:site_name").unwrap_or_else(|| "Facebook".to_string()),
            video_url,
            audio_url: None,
            thumbnail_url: common::extract_meta_content(&html, "og:image").unwrap_or_default(),
            original_platform: "facebook".to_string(),
            file_size_bytes: None,
        })
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        let metadata = self.fetch_metadata(url).await?;
        Ok(metadata.video_url)
    }
}
