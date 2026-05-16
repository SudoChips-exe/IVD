use async_trait::async_trait;
use crate::api::common;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use crate::util;
use serde_json::Value;
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
        let html = common::fetch_html(url).await?;
        self.parse_tiktok_metadata(&html, url)
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        let metadata = self.fetch_metadata(url).await?;
        Ok(metadata.video_url)
    }
}

impl TikTokAdapter {
    fn parse_tiktok_metadata(&self, html: &str, url: &str) -> AppResult<VideoMetadata> {
        if let Some(metadata) = self.extract_metadata_from_state(html) {
            return Ok(metadata);
        }

        if let Some(video_url) = common::extract_meta_content(html, "og:video:secure_url")
            .or_else(|| common::extract_meta_content(html, "og:video"))
        {
            return Ok(VideoMetadata {
                title: common::extract_meta_content(html, "og:title").unwrap_or_else(|| "TikTok Video".to_string()),
                duration_seconds: 0,
                author: common::extract_meta_content(html, "og:description").unwrap_or_else(|| "TikTok".to_string()),
                video_url,
                audio_url: None,
                thumbnail_url: common::extract_meta_content(html, "og:image").unwrap_or_default(),
                original_platform: "tiktok".to_string(),
                file_size_bytes: None,
            });
        }

        let video_id = util::extract_video_id(url, crate::models::Platform::TikTok)?;
        Err(AppError::VideoNotFound(format!(
            "Could not extract TikTok metadata for video ID {}",
            video_id
        )))
    }

    fn extract_metadata_from_state(&self, html: &str) -> Option<VideoMetadata> {
        let json_text = common::extract_script_json(html, "SIGI_STATE")?;
        let state = common::parse_json_value(&json_text)?;
        let item_module = state.get("ItemModule")?.as_object()?;

        for item in item_module.values() {
            let video = item.get("video")?;
            let video_url = self
                .extract_string(video, &["downloadAddr"])
                .or_else(|| self.extract_string(video, &["playAddr"]))?;

            let title = item
                .get("desc")
                .and_then(|v| v.as_str())
                .unwrap_or("TikTok Video")
                .to_string();

            let author = item
                .get("author")
                .and_then(|author| author.get("nickname"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "TikTok".to_string());

            let duration_seconds = item
                .get("video")
                .and_then(|video| video.get("duration"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;

            let thumbnail_url = item
                .get("video")
                .and_then(|video| video.get("cover"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            return Some(VideoMetadata {
                title,
                duration_seconds,
                author,
                video_url,
                audio_url: None,
                thumbnail_url,
                original_platform: "tiktok".to_string(),
                file_size_bytes: None,
            });
        }

        None
    }

    fn extract_string(&self, value: &Value, keys: &[&str]) -> Option<String> {
        let mut current = value;
        for key in keys {
            current = current.get(key)?;
        }

        current.as_str().map(|s| s.to_string())
    }
}
