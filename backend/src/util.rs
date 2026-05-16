use crate::models::Platform;
use crate::error::{AppError, AppResult};
use regex::Regex;

/// Validate if a URL is a valid social media URL
pub fn validate_url(url: &str) -> AppResult<()> {
    if url.trim().is_empty() {
        return Err(AppError::InvalidUrl("URL cannot be empty".to_string()));
    }

    if url.len() > 2048 {
        return Err(AppError::InvalidUrl("URL is too long (max 2048 characters)".to_string()));
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AppError::InvalidUrl("URL must start with http:// or https://".to_string()));
    }

    Ok(())
}

/// Detect platform from URL
pub fn detect_platform(url: &str) -> Platform {
    let url_lower = url.to_lowercase();

    if url_lower.contains("instagram.com") || url_lower.contains("ig.me") {
        Platform::Instagram
    } else if url_lower.contains("tiktok.com") || url_lower.contains("vm.tiktok.com") || url_lower.contains("vt.tiktok.com") {
        Platform::TikTok
    } else if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") {
        Platform::YouTube
    } else if url_lower.contains("twitter.com") || url_lower.contains("x.com") {
        Platform::Twitter
    } else if url_lower.contains("facebook.com") || url_lower.contains("fb.watch") {
        Platform::Facebook
    } else if url_lower.contains("snapchat.com") || url_lower.contains("snap.com") {
        Platform::Snapchat
    } else {
        Platform::Unknown
    }
}

/// Extract video ID from URL based on platform
pub fn extract_video_id(url: &str, platform: Platform) -> AppResult<String> {
    let url = url.trim();

    match platform {
        Platform::Instagram => {
            // Instagram: /p/ID or /reel/ID
            let re = Regex::new(r"(?:instagram\.com|ig\.me)/(?:p|reel|tv)/([a-zA-Z0-9_-]+)")
                .unwrap();
            if let Some(caps) = re.captures(url) {
                Ok(caps.get(1).unwrap().as_str().to_string())
            } else {
                Err(AppError::InvalidUrl(
                    "Could not extract Instagram video ID".to_string(),
                ))
            }
        }
        Platform::TikTok => {
            // TikTok: /@user/video/ID or short links
            let re = Regex::new(r"(?:tiktok\.com/(?:@[^/]+/video/|[^/]+/video/)?|vm\.tiktok\.com|vt\.tiktok\.com)(?:video/)?(\d+)")
                .unwrap();
            if let Some(caps) = re.captures(url) {
                Ok(caps.get(1).unwrap().as_str().to_string())
            } else {
                Err(AppError::InvalidUrl(
                    "Could not extract TikTok video ID".to_string(),
                ))
            }
        }
        Platform::YouTube => {
            // YouTube: watch?v=ID, youtu.be/ID, shorts/ID, embed/ID
            let re = Regex::new(
                r"(?:youtube\.com/watch\?v=|youtu\.be/|youtube\.com/shorts/|youtube\.com/embed/)([a-zA-Z0-9_-]{11})",
            )
            .unwrap();
            if let Some(caps) = re.captures(url) {
                Ok(caps.get(1).unwrap().as_str().to_string())
            } else {
                Err(AppError::InvalidUrl(
                    "Could not extract YouTube video ID".to_string(),
                ))
            }
        }
        Platform::Twitter => {
            // Twitter: /status/ID
            let re = Regex::new(r"(?:twitter\.com|x\.com)/\w+/status/(\d+)")
                .unwrap();
            if let Some(caps) = re.captures(url) {
                Ok(caps.get(1).unwrap().as_str().to_string())
            } else {
                Err(AppError::InvalidUrl(
                    "Could not extract Twitter video ID".to_string(),
                ))
            }
        }
        Platform::Facebook => {
            // Facebook: various patterns
            if let Some(pos) = url.find("video.php") {
                let query = &url[pos..];
                if let Some(start) = query.find("v=") {
                    let id_start = start + 2;
                    let id = query[id_start..]
                        .split('&')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    if !id.is_empty() {
                        return Ok(id);
                    }
                }
            }
            if let Some(pos) = url.find("watch") {
                let query = &url[pos..];
                if let Some(start) = query.find("v=") {
                    let id_start = start + 2;
                    let id = query[id_start..]
                        .split('&')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    if !id.is_empty() {
                        return Ok(id);
                    }
                }
            }
            Err(AppError::InvalidUrl(
                "Could not extract Facebook video ID".to_string(),
            ))
        }
        Platform::Snapchat => {
            // Snapchat: /add/ID or similar
            let re = Regex::new(r"snapchat\.com/(?:add|clip)/([a-zA-Z0-9_-]+)")
                .unwrap();
            if let Some(caps) = re.captures(url) {
                Ok(caps.get(1).unwrap().as_str().to_string())
            } else {
                Err(AppError::InvalidUrl(
                    "Could not extract Snapchat ID".to_string(),
                ))
            }
        }
        Platform::Unknown => {
            Err(AppError::PlatformNotSupported(
                "Unknown platform".to_string(),
            ))
        }
    }
}

/// Generate a filename for downloaded video
pub fn generate_filename(platform: &Platform, video_id: &str) -> String {
    format!(
        "{}_{}_downloaded.mp4",
        platform.as_str(),
        video_id.chars().take(10).collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_platform_instagram() {
        assert_eq!(
            detect_platform("https://www.instagram.com/p/ABC123/"),
            Platform::Instagram
        );
    }

    #[test]
    fn test_detect_platform_tiktok() {
        assert_eq!(
            detect_platform("https://www.tiktok.com/video/123456789"),
            Platform::TikTok
        );
    }

    #[test]
    fn test_detect_platform_youtube() {
        assert_eq!(
            detect_platform("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Platform::YouTube
        );
    }

    #[test]
    fn test_extract_video_id_instagram() {
        let id = extract_video_id(
            "https://www.instagram.com/p/ABC123def456/",
            Platform::Instagram,
        )
        .unwrap();
        assert_eq!(id, "ABC123def456");
    }

    #[test]
    fn test_generate_filename() {
        let filename = generate_filename(&Platform::Instagram, "ABC123def456");
        assert!(filename.contains("instagram"));
        assert!(filename.contains("ABC123de"));
        assert!(filename.ends_with(".mp4"));
    }
}
