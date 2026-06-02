use crate::error::{AppError, AppResult};
#[cfg(test)]
use crate::models::Platform;
#[cfg(test)]
use regex::Regex;

/// Validate if a URL is a valid social media URL
pub fn validate_url(url: &str) -> AppResult<()> {
    let url = url.trim();

    if url.is_empty() {
        return Err(AppError::InvalidUrl("URL cannot be empty".to_string()));
    }

    if url.len() > 2048 {
        return Err(AppError::InvalidUrl("URL is too long (max 2048 characters)".to_string()));
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AppError::InvalidUrl(format!(
            "URL must start with http:// or https:// (got: {:?})",
            &url[..url.len().min(40)]
        )));
    }

    Ok(())
}

#[cfg(test)]
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
    } else {
        Platform::Unknown
    }
}

#[cfg(test)]
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
        Platform::Unknown => {
            Err(AppError::PlatformNotSupported(
                "Unknown platform".to_string(),
            ))
        }
    }
}

#[cfg(test)]
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

    // ── validate_url ──────────────────────────────────────────────────────────

    #[test]
    fn validate_url_rejects_empty() {
        assert!(validate_url("").is_err());
        assert!(validate_url("   ").is_err());
    }

    #[test]
    fn validate_url_rejects_no_protocol() {
        assert!(validate_url("instagram.com/reel/abc").is_err());
    }

    #[test]
    fn validate_url_rejects_too_long() {
        let long = format!("https://instagram.com/{}", "a".repeat(2048));
        assert!(validate_url(&long).is_err());
    }

    #[test]
    fn validate_url_accepts_http_and_https() {
        assert!(validate_url("https://www.instagram.com/reel/abc").is_ok());
        assert!(validate_url("http://www.tiktok.com/video/123").is_ok());
    }

    // ── detect_platform ───────────────────────────────────────────────────────

    #[test]
    fn detect_instagram() {
        assert_eq!(detect_platform("https://www.instagram.com/p/ABC123/"), Platform::Instagram);
        assert_eq!(detect_platform("https://ig.me/p/ABC123"), Platform::Instagram);
    }

    #[test]
    fn detect_tiktok() {
        assert_eq!(detect_platform("https://www.tiktok.com/@user/video/123"), Platform::TikTok);
        assert_eq!(detect_platform("https://vm.tiktok.com/abc"), Platform::TikTok);
        assert_eq!(detect_platform("https://vt.tiktok.com/abc"), Platform::TikTok);
    }

    #[test]
    fn detect_youtube() {
        assert_eq!(detect_platform("https://www.youtube.com/watch?v=dQw4w9WgXcQ"), Platform::YouTube);
        assert_eq!(detect_platform("https://youtu.be/dQw4w9WgXcQ"), Platform::YouTube);
    }

    #[test]
    fn detect_twitter_and_x() {
        assert_eq!(detect_platform("https://twitter.com/user/status/123"), Platform::Twitter);
        assert_eq!(detect_platform("https://x.com/user/status/123"), Platform::Twitter);
    }

    #[test]
    fn detect_facebook() {
        assert_eq!(detect_platform("https://www.facebook.com/video/123"), Platform::Facebook);
        assert_eq!(detect_platform("https://fb.watch/abc123"), Platform::Facebook);
    }

    #[test]
    fn detect_unknown() {
        assert_eq!(detect_platform("https://example.com/video/123"), Platform::Unknown);
    }

    #[test]
    fn detect_is_case_insensitive() {
        assert_eq!(detect_platform("https://INSTAGRAM.COM/reel/abc"), Platform::Instagram);
        assert_eq!(detect_platform("HTTPS://YOUTUBE.COM/watch?v=abc"), Platform::YouTube);
    }

    // ── extract_video_id ──────────────────────────────────────────────────────

    #[test]
    fn extract_instagram_reel_id() {
        let id = extract_video_id("https://www.instagram.com/reel/CxYZ123abc/", Platform::Instagram).unwrap();
        assert_eq!(id, "CxYZ123abc");
    }

    #[test]
    fn extract_instagram_post_id() {
        let id = extract_video_id("https://www.instagram.com/p/ABC123def456/", Platform::Instagram).unwrap();
        assert_eq!(id, "ABC123def456");
    }

    #[test]
    fn extract_youtube_watch_id() {
        let id = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ", Platform::YouTube).unwrap();
        assert_eq!(id, "dQw4w9WgXcQ");
    }

    #[test]
    fn extract_youtube_short_url_id() {
        let id = extract_video_id("https://youtu.be/dQw4w9WgXcQ", Platform::YouTube).unwrap();
        assert_eq!(id, "dQw4w9WgXcQ");
    }

    #[test]
    fn extract_tiktok_video_id() {
        let id = extract_video_id("https://www.tiktok.com/@user/video/7123456789012345678", Platform::TikTok).unwrap();
        assert_eq!(id, "7123456789012345678");
    }

    #[test]
    fn extract_twitter_status_id() {
        let id = extract_video_id("https://twitter.com/user/status/1234567890123", Platform::Twitter).unwrap();
        assert_eq!(id, "1234567890123");
    }

    #[test]
    fn extract_unknown_platform_errors() {
        assert!(extract_video_id("https://example.com/video", Platform::Unknown).is_err());
    }

    // ── generate_filename ─────────────────────────────────────────────────────

    #[test]
    fn filename_format_is_correct() {
        let f = generate_filename(&Platform::Instagram, "ABC123def456");
        assert_eq!(f, "instagram_ABC123def4_downloaded.mp4");
    }

    #[test]
    fn filename_truncates_long_id() {
        let f = generate_filename(&Platform::YouTube, "averylongvideoidthatexceedstenlength");
        assert_eq!(f, "youtube_averylongv_downloaded.mp4");
    }

    #[test]
    fn filename_handles_short_id() {
        let f = generate_filename(&Platform::TikTok, "short");
        assert_eq!(f, "tiktok_short_downloaded.mp4");
    }
}
