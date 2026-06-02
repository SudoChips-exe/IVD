use tokio::process::Command;
use crate::error::{AppError, AppResult};

const BROWSERS: &[&str] = &["chrome", "chromium", "firefox", "brave", "edge"];

pub struct ExtractedVideo {
    pub file_path: String,
    pub filename: String,
}

pub async fn extract(url: &str) -> AppResult<ExtractedVideo> {
    let id = uuid::Uuid::new_v4().to_string();
    let tmp_path = format!("/tmp/vidclaw_{}.mp4", id);

    let title = get_title(url).await;
    let filename = build_filename(title.as_deref());

    // Try without cookies first (works for YouTube, TikTok, public content)
    if run_ytdlp(url, &tmp_path, None).await {
        return verify_and_return(tmp_path, filename).await;
    }

    // Fall back: try each browser's cookies (needed for Instagram, Facebook, etc.)
    log::info!("yt-dlp failed without cookies — trying browser cookies for {}", url);
    for browser in BROWSERS {
        let _ = tokio::fs::remove_file(&tmp_path).await;
        if run_ytdlp(url, &tmp_path, Some(browser)).await {
            log::info!("yt-dlp succeeded with {} cookies", browser);
            return verify_and_return(tmp_path, filename).await;
        }
    }

    let _ = tokio::fs::remove_file(&tmp_path).await;
    Err(AppError::PlatformError(
        "Could not download this video. The content may be private, \
         geo-restricted, or require logging into the platform in your browser."
            .to_string(),
    ))
}

async fn run_ytdlp(url: &str, tmp_path: &str, browser: Option<&str>) -> bool {
    let mut args: Vec<&str> = vec![
        "-o",
        tmp_path,
        "-f",
        "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
        "--merge-output-format",
        "mp4",
        "--no-playlist",
        "--quiet",
        "--no-warnings",
    ];

    if let Some(b) = browser {
        args.push("--cookies-from-browser");
        args.push(b);
    }

    args.push(url);

    Command::new("yt-dlp")
        .args(&args)
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

async fn verify_and_return(file_path: String, filename: String) -> AppResult<ExtractedVideo> {
    let meta = tokio::fs::metadata(&file_path).await.map_err(|_| {
        AppError::VideoNotFound("Download completed but output file not found".to_string())
    })?;

    if meta.len() == 0 {
        let _ = tokio::fs::remove_file(&file_path).await;
        return Err(AppError::VideoNotFound(
            "Downloaded file is empty".to_string(),
        ));
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
        .args([
            "--get-title",
            "--no-playlist",
            "--quiet",
            "--no-warnings",
            url,
        ])
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
