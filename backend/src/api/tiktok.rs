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

        if let Some(metadata) = self.extract_metadata_from_universal_data(html) {
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

    fn extract_metadata_from_universal_data(&self, html: &str) -> Option<VideoMetadata> {
        let json_text = common::extract_script_json(html, "__UNIVERSAL_DATA_FOR_REHYDRATION__")?;
        let state = common::parse_json_value(&json_text)?;

        let default_scope = state.get("__DEFAULT_SCOPE__")?;
        let video_detail = default_scope.get("webapp.video-detail")?;
        let item_info = video_detail.get("itemInfo")?;
        let item_struct = item_info.get("itemStruct")?;
        let video = item_struct.get("video")?;

        let video_url = self
            .extract_tiktok_video_url(video)
            .or_else(|| self.extract_string(video, &["downloadAddr"]))
            .or_else(|| self.extract_string(video, &["playAddr"]))?;

        log::info!("TikTok universal data fallback found video_url={}", video_url);

        let title = item_struct
            .get("desc")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "TikTok Video".to_string());

        let author = item_struct
            .get("author")
            .and_then(|author| author.get("nickname"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "TikTok".to_string());

        let duration_seconds = video
            .get("duration")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        let thumbnail_url = self
            .extract_string(video, &["cover"])
            .or_else(|| self.extract_string(video, &["originCover"]))
            .unwrap_or_default();

        let file_size_bytes = video.get("size").and_then(|v| v.as_u64());

        Some(VideoMetadata {
            title,
            duration_seconds,
            author,
            video_url,
            audio_url: None,
            thumbnail_url,
            original_platform: "tiktok".to_string(),
            file_size_bytes,
        })
    }

    fn extract_metadata_from_state(&self, html: &str) -> Option<VideoMetadata> {
        let json_text = common::extract_script_json(html, "SIGI_STATE")?;
        let state = common::parse_json_value(&json_text)?;
        let item_module = state.get("ItemModule")?.as_object()?;

        for item in item_module.values() {
            let video = item.get("video")?;
            let video_url = self
                .extract_tiktok_video_url(video)
                .or_else(|| self.extract_string(video, &["downloadAddr"]))
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

    fn extract_tiktok_video_url(&self, video: &Value) -> Option<String> {
        self.extract_url_from_value(video.get("PlayAddrStruct"))
            .or_else(|| self.extract_url_from_value(video.get("playAddr")))
            .or_else(|| self.extract_url_from_value(video.get("downloadAddr")))
    }

    fn extract_url_from_value(&self, value: Option<&Value>) -> Option<String> {
        let value = value?;

        if let Some(url) = value.as_str() {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                return Some(self.normalize_tiktok_url(trimmed));
            }
        }

        if let Some(urls) = value.as_array() {
            let normalized_urls: Vec<String> = urls
                .iter()
                .filter_map(|item| item.as_str())
                .map(|url| self.normalize_tiktok_url(url.trim()))
                .filter(|url| !url.is_empty())
                .collect();

            if let Some(url) = normalized_urls.iter().find(|url| url.contains("/aweme/v1/play/")) {
                return Some(url.clone());
            }
            return normalized_urls.into_iter().next();
        }

        if let Some(obj) = value.as_object() {
            if let Some(urls) = obj.get("UrlList").and_then(|v| v.as_array()) {
                let normalized_urls: Vec<String> = urls
                    .iter()
                    .filter_map(|item| item.as_str())
                    .map(|url| self.normalize_tiktok_url(url.trim()))
                    .filter(|url| !url.is_empty())
                    .collect();

                if let Some(url) = normalized_urls.iter().find(|url| url.contains("/aweme/v1/play/")) {
                    return Some(url.clone());
                }
                if let Some(url) = normalized_urls.first() {
                    return Some(url.clone());
                }
            }

            if let Some(url) = obj.get("Uri").and_then(|v| v.as_str()) {
                let trimmed = url.trim();
                if !trimmed.is_empty() && self.is_valid_tiktok_url(trimmed) {
                    return Some(self.normalize_tiktok_url(trimmed));
                }
            }

            if let Some(url) = obj.get("uri").and_then(|v| v.as_str()) {
                let trimmed = url.trim();
                if !trimmed.is_empty() && self.is_valid_tiktok_url(trimmed) {
                    return Some(self.normalize_tiktok_url(trimmed));
                }
            }
        }

        None
    }

    fn normalize_tiktok_url(&self, url: &str) -> String {
        if url.starts_with("//") {
            format!("https:{}", url)
        } else if url.starts_with("/aweme/v1/play/") {
            format!("https://www.tiktok.com{}", url)
        } else {
            url.to_string()
        }
    }

    fn is_valid_tiktok_url(&self, url: &str) -> bool {
        url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("//")
            || url.starts_with("/aweme/v1/play/")
    }

    fn extract_string(&self, value: &Value, keys: &[&str]) -> Option<String> {
        let mut current = value;
        for key in keys {
            current = current.get(key)?;
        }

        current.as_str().and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
    }
}
