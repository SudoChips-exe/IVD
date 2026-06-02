use tokio::process::Command;
use crate::error::{AppError, AppResult};

const BROWSERS: &[&str] = &["chrome", "chromium", "firefox", "brave", "edge"];
const COOKIES_FILE: &str = "~/.config/vidclaw/cookies.txt";

pub struct ExtractedVideo {
    pub file_path: String,
    pub filename: String,
}

pub async fn extract(url: &str) -> AppResult<ExtractedVideo> {
    let id = uuid::Uuid::new_v4().to_string();
    let tmp_path = format!("/tmp/vidclaw_{}.mp4", id);

    let title = get_title(url).await;
    let filename = build_filename(title.as_deref());

    // Try without cookies first (YouTube, TikTok, public content)
    match run_ytdlp(url, &tmp_path, CookieSource::None).await {
        Ok(()) => return verify_and_return(tmp_path, filename).await,
        Err(ref stderr) => {
            // Fail fast on errors that won't be fixed by adding cookies
            if let Some(hard_err) = classify_hard_error(stderr) {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                return Err(hard_err);
            }
        }
    }

    // Try user-supplied cookies.txt (most reliable for Instagram/Facebook)
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        log::info!("Trying cookies file: {}", cookies_path);
        let _ = tokio::fs::remove_file(&tmp_path).await;
        if run_ytdlp(url, &tmp_path, CookieSource::File(&cookies_path)).await.is_ok() {
            log::info!("yt-dlp succeeded with cookies file");
            return verify_and_return(tmp_path, filename).await;
        }
    }

    // Fall back through browser cookies (Chrome, Chromium, Firefox, Brave, Edge)
    log::info!("yt-dlp failed without cookies — trying browser cookies for {}", url);
    let mut last_stderr = String::new();
    for browser in BROWSERS {
        let _ = tokio::fs::remove_file(&tmp_path).await;
        match run_ytdlp(url, &tmp_path, CookieSource::Browser(browser)).await {
            Ok(()) => {
                log::info!("yt-dlp succeeded with {} cookies", browser);
                return verify_and_return(tmp_path, filename).await;
            }
            Err(stderr) => {
                if !stderr.is_empty() && !stderr.contains("could not find") {
                    last_stderr = stderr;
                }
            }
        }
    }

    let _ = tokio::fs::remove_file(&tmp_path).await;
    Err(classify_auth_error(&last_stderr))
}

enum CookieSource<'a> {
    None,
    File(&'a str),
    Browser(&'a str),
}

/// Returns Ok(()) on success, Err(stderr) on failure.
async fn run_ytdlp(url: &str, tmp_path: &str, cookies: CookieSource<'_>) -> Result<(), String> {
    let mut args: Vec<String> = vec![
        "-o".into(),
        tmp_path.into(),
        "-f".into(),
        "bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best[ext=mp4]/best".into(),
        "--merge-output-format".into(),
        "mp4".into(),
        "--no-playlist".into(),
        "--quiet".into(),
    ];

    match cookies {
        CookieSource::None => {}
        CookieSource::File(path) => {
            args.push("--cookies".into());
            args.push(path.into());
        }
        CookieSource::Browser(browser) => {
            args.push("--cookies-from-browser".into());
            args.push(browser.into());
        }
    }

    args.push(url.into());

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if !stderr.trim().is_empty() {
            log::warn!("yt-dlp stderr: {}", stderr.trim());
        }
        Err(stderr)
    }
}

/// Errors that cookies won't fix — fail fast instead of retrying all browsers.
fn classify_hard_error(stderr: &str) -> Option<AppError> {
    let s = stderr.to_lowercase();
    if s.contains("video unavailable") || s.contains("has been removed") || s.contains("no longer available") {
        Some(AppError::VideoNotFound("This video is no longer available.".to_string()))
    } else if s.contains("private video") || s.contains("private post") {
        Some(AppError::VideoNotFound("This video is private.".to_string()))
    } else if s.contains("age-restricted") || s.contains("age restricted") {
        Some(AppError::PlatformError(
            "Age-restricted content. Log into the platform in your browser and retry.".to_string(),
        ))
    } else if s.contains("not available in your country") || s.contains("geo") {
        Some(AppError::PlatformError(
            "This video is geo-restricted and not available in your region.".to_string(),
        ))
    } else if s.contains("copyright") {
        Some(AppError::PlatformError(
            "This video is unavailable due to a copyright claim.".to_string(),
        ))
    } else {
        None
    }
}

/// Error shown after all cookie strategies failed.
fn classify_auth_error(stderr: &str) -> AppError {
    let s = stderr.to_lowercase();
    if s.contains("rate") || s.contains("too many requests") || s.contains("429") {
        AppError::PlatformError(
            "Rate limited by the platform. Wait a few minutes and try again.".to_string(),
        )
    } else if s.contains("login") || s.contains("sign in") || s.contains("authentication") || s.contains("empty media response") {
        AppError::PlatformError(
            "This video requires you to be logged in. \
             Log into the platform in Chromium or Brave, or export cookies to \
             ~/.config/vidclaw/cookies.txt using the 'Get cookies.txt LOCALLY' browser extension."
                .to_string(),
        )
    } else {
        AppError::PlatformError(
            "Could not download this video. It may be private, geo-restricted, or \
             require authentication. Check the URL and try again."
                .to_string(),
        )
    }
}

fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}{}", home, &path[1..]);
        }
    }
    path.to_string()
}

async fn verify_and_return(file_path: String, filename: String) -> AppResult<ExtractedVideo> {
    let meta = tokio::fs::metadata(&file_path).await.map_err(|_| {
        AppError::VideoNotFound("Download completed but output file not found".to_string())
    })?;

    if meta.len() == 0 {
        let _ = tokio::fs::remove_file(&file_path).await;
        return Err(AppError::VideoNotFound("Downloaded file is empty".to_string()));
    }

    Ok(ExtractedVideo { file_path, filename })
}

fn build_filename(title: Option<&str>) -> String {
    match title {
        Some(t) => {
            let safe: String = t
                .chars()
                .map(|c| match c {
                    '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                    c => c,
                })
                .take(80)
                .collect();
            format!("{}.mp4", safe)
        }
        None => "video.mp4".to_string(),
    }
}

async fn get_title(url: &str) -> Option<String> {
    let output = Command::new("yt-dlp")
        .args(["--get-title", "--no-playlist", "--quiet", "--no-warnings", url])
        .output()
        .await
        .ok()?;

    if output.status.success() {
        let t = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !t.is_empty() {
            return Some(t);
        }
    }
    None
}
