use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Instagram,
    TikTok,
    YouTube,
    Twitter,
    Facebook,
    Unknown,
}

#[allow(dead_code)]
impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Instagram => "instagram",
            Platform::TikTok => "tiktok",
            Platform::YouTube => "youtube",
            Platform::Twitter => "twitter",
            Platform::Facebook => "facebook",
            Platform::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "instagram" => Platform::Instagram,
            "tiktok" => Platform::TikTok,
            "youtube" => Platform::YouTube,
            "twitter" | "x" => Platform::Twitter,
            "facebook" => Platform::Facebook,
            _ => Platform::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadRequest {
    pub url: String,
    pub quality: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoMetadata {
    pub title: String,
    pub duration_seconds: u32,
    pub author: String,
    pub video_url: String,
    pub audio_url: Option<String>,
    pub thumbnail_url: String,
    pub original_platform: String,
    pub file_size_bytes: Option<u64>,
}
