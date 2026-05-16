use async_trait::async_trait;
use crate::api::common;
use crate::error::{AppError, AppResult};
use crate::models::VideoMetadata;
use super::PlatformAdapter;
use regex::Regex;
use serde_json::Value;

pub struct InstagramAdapter;

#[async_trait]
impl PlatformAdapter for InstagramAdapter {
    async fn validate_url(&self, url: &str) -> AppResult<bool> {
        let url_lower = url.to_lowercase();
        Ok(url_lower.contains("instagram.com") || url_lower.contains("ig.me"))
    }

    async fn fetch_metadata(&self, url: &str) -> AppResult<VideoMetadata> {
        let html = common::fetch_html(url).await?;
        self.parse_instagram_metadata(&html)
    }

    async fn get_download_url(&self, url: &str) -> AppResult<String> {
        let metadata = self.fetch_metadata(url).await?;
        Ok(metadata.video_url)
    }
}

impl InstagramAdapter {
    fn parse_instagram_metadata(&self, html: &str) -> AppResult<VideoMetadata> {
        if let Some(metadata) = self.extract_metadata_from_ld_json(html) {
            return Ok(metadata);
        }

        if let Some(video_url) = self.extract_video_url_from_meta(html) {
            return Ok(VideoMetadata {
                title: "Instagram Video".to_string(),
                duration_seconds: 0,
                author: "Instagram".to_string(),
                video_url,
                audio_url: None,
                thumbnail_url: String::new(),
                original_platform: "instagram".to_string(),
                file_size_bytes: None,
            });
        }

        Err(AppError::VideoNotFound(
            "Could not extract Instagram video metadata".to_string(),
        ))
    }

    fn extract_metadata_from_ld_json(&self, html: &str) -> Option<VideoMetadata> {
        let scripts = common::extract_ld_json_scripts(html);
        for script in scripts {
            let value = match serde_json::from_str::<Value>(&script) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if let Some(metadata) = self.parse_metadata_value(&value) {
                return Some(metadata);
            }

            if let Some(array) = value.as_array() {
                for item in array {
                    if let Some(metadata) = self.parse_metadata_value(item) {
                        return Some(metadata);
                    }
                }
            }
        }
        None
    }

    fn parse_metadata_value(&self, value: &Value) -> Option<VideoMetadata> {
        let has_video = value.get("contentUrl").is_some() || value.get("url").is_some();
        if !has_video {
            return None;
        }

        let video_url = self
            .extract_string(value, &["contentUrl"])
            .or_else(|| self.extract_string(value, &["url"]))
            .or_else(|| self.extract_string(value, &["video_url"]))?;

        let title = self
            .extract_string(value, &["name"])
            .unwrap_or_else(|| "Instagram Video".to_string());

        let thumbnail_url = self
            .extract_string(value, &["thumbnailUrl"])
            .unwrap_or_default();

        let author = self.extract_author(value);
        let duration_seconds = value
            .get("duration")
            .and_then(|v| v.as_str())
            .map(Self::parse_iso_duration)
            .unwrap_or(0);

        Some(VideoMetadata {
            title,
            duration_seconds,
            author,
            video_url,
            audio_url: None,
            thumbnail_url,
            original_platform: "instagram".to_string(),
            file_size_bytes: None,
        })
    }

    fn extract_string(&self, value: &Value, path: &[&str]) -> Option<String> {
        let mut current = value;
        for key in path {
            current = current.get(key)?;
        }

        if let Some(s) = current.as_str() {
            return Some(s.to_string());
        }

        if let Some(array) = current.as_array() {
            return array
                .iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .next();
        }

        if let Some(obj) = current.as_object() {
            return obj
                .get("name")
                .and_then(|name| name.as_str())
                .map(|s| s.to_string());
        }

        None
    }

    fn extract_author(&self, value: &Value) -> String {
        if let Some(author) = value.get("author") {
            if let Some(name) = author.get("name").and_then(|v| v.as_str()) {
                return name.to_string();
            }
            if let Some(author_str) = author.as_str() {
                return author_str.to_string();
            }
        }
        "Instagram".to_string()
    }

    fn parse_iso_duration(duration: &str) -> u32 {
        let re = Regex::new(r"^PT(?:(?P<h>\d+)H)?(?:(?P<m>\d+)M)?(?:(?P<s>\d+)S)?$").unwrap();
        if let Some(caps) = re.captures(duration) {
            let hours = caps.name("h").and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);
            let minutes = caps.name("m").and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);
            let seconds = caps.name("s").and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(0);
            return hours * 3600 + minutes * 60 + seconds;
        }
        0
    }

    fn extract_video_url_from_meta(&self, html: &str) -> Option<String> {
        let og_video_re = Regex::new(r#"<meta[^>]*property=["']og:video:secure_url["'][^>]*content=["']([^"']+)["'][^>]*>"#).unwrap();
        if let Some(caps) = og_video_re.captures(html) {
            return caps.get(1).map(|m| m.as_str().to_string());
        }

        let og_video_re = Regex::new(r#"<meta[^>]*property=["']og:video["'][^>]*content=["']([^"']+)["'][^>]*>"#).unwrap();
        if let Some(caps) = og_video_re.captures(html) {
            return caps.get(1).map(|m| m.as_str().to_string());
        }

        let generic_re = Regex::new(r#"["']video_url["']\s*:\s*["']([^"']+)["']"#).unwrap();
        generic_re
            .captures(html)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }
}
