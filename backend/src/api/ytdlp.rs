use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, Mutex};

use crate::error::{AppError, AppResult};
use crate::jobs::{JobEvent, JobResult};
use crate::models::VideoInfo;

const BROWSERS: &[&str] = &["chrome", "chromium", "firefox", "brave", "edge"];
const COOKIES_FILE: &str = "~/.config/vidclaw/cookies.txt";

/// Build a minimal Netscape cookies.txt from environment-variable session tokens.
/// Supports INSTAGRAM_SESSION_ID and FACEBOOK_SESSION_COOKIES (semicolon-separated key=value pairs).
async fn build_session_cookies(url: &str) -> Option<String> {
    let is_instagram = url.contains("instagram.com") || url.contains("ig.me");
    let is_facebook  = url.contains("facebook.com")  || url.contains("fb.watch");

    if !is_instagram && !is_facebook {
        return None;
    }

    let mut lines = vec!["# Netscape HTTP Cookie File".to_string()];
    let mut has_cookies = false;

    if is_instagram {
        if let Ok(session_id) = std::env::var("INSTAGRAM_SESSION_ID") {
            if !session_id.is_empty() {
                lines.push(format!(".instagram.com\tTRUE\t/\tTRUE\t9999999999\tsessionid\t{}", session_id));
                has_cookies = true;
            }
        }
    }

    if is_facebook {
        // FACEBOOK_SESSION_COOKIES = semicolon-separated key=value pairs
        // e.g. "c_user=123;xs=abc;fr=xyz"
        if let Ok(fb_cookies) = std::env::var("FACEBOOK_SESSION_COOKIES") {
            for pair in fb_cookies.split(';') {
                let pair = pair.trim();
                if let Some((k, v)) = pair.split_once('=') {
                    lines.push(format!(".facebook.com\tTRUE\t/\tTRUE\t9999999999\t{}\t{}", k.trim(), v.trim()));
                    has_cookies = true;
                }
            }
        }
    }

    if !has_cookies {
        return None;
    }

    // Write temp cookies file
    let tmp = format!("/tmp/vidclaw_cookies_{}.txt", uuid::Uuid::new_v4());
    tokio::fs::write(&tmp, lines.join("\n") + "\n").await.ok()?;
    Some(tmp)
}

pub async fn get_info(url: &str) -> AppResult<VideoInfo> {
    // Try without auth, then session cookies, then cookies file
    if let Some(info) = try_get_info(url, None).await {
        return Ok(info);
    }
    if let Some(session_file) = build_session_cookies(url).await {
        let result = try_get_info(url, Some(&session_file)).await;
        let _ = tokio::fs::remove_file(&session_file).await;
        if let Some(info) = result { return Ok(info); }
    }
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        if let Some(info) = try_get_info(url, Some(&cookies_path)).await {
            return Ok(info);
        }
        // Fallback: --print skips format selection entirely (succeeds even when YouTube
        // restricts format URLs to datacenter IPs due to po_token requirements)
        if let Some(info) = try_get_info_nofmt(url, Some(&cookies_path)).await {
            return Ok(info);
        }
    }
    // Last resort: --print without cookies (gets metadata if video is public)
    if let Some(info) = try_get_info_nofmt(url, None).await {
        return Ok(info);
    }
    Err(AppError::PlatformError("Could not fetch video info. Check the URL or platform support.".to_string()))
}

async fn try_get_info(url: &str, cookies: Option<&str>) -> Option<VideoInfo> {
    let mut args = vec![
        "--dump-json".to_string(),
        "--no-playlist".to_string(),
        "--quiet".to_string(),
        "--no-warnings".to_string(),
    ];
    // bgutil generates po_tokens for web-based clients only (not android_vr/ios).
    // mweb,web ensures bgutil can provide po_token from datacenter IPs.
    if cookies.is_some() {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=web".to_string());
    } else {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=mweb,web".to_string());
    }
    // Pass bgutil script provider; server_home must point to server/ subdir (plugin appends build/)
    args.push("--extractor-args".to_string());
    args.push("youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".to_string());
    if let Some(path) = cookies {
        args.push("--cookies".to_string());
        args.push(path.to_string());
    }
    if let Some(proxy) = get_proxy() {
        args.push("--proxy".to_string());
        args.push(proxy);
    }
    args.push(url.to_string());

    let output = Command::new(ytdlp_bin())
        .args(&args)
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            log::warn!("yt-dlp info failed for {}: {}", url, stderr.trim());
        }
        return None;
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;

    let thumbnail = json["thumbnail"].as_str()
        .or_else(|| json["thumbnails"].as_array()?.last()?.get("url")?.as_str())
        .map(str::to_string);

    Some(VideoInfo {
        title: json["title"].as_str().unwrap_or("Unknown").to_string(),
        uploader: json["uploader"].as_str()
            .or_else(|| json["channel"].as_str())
            .or_else(|| json["creator"].as_str())
            .unwrap_or("Unknown")
            .to_string(),
        duration_seconds: json["duration"].as_u64(),
        thumbnail_url: thumbnail,
        filesize_approx: json["filesize_approx"].as_u64().or_else(|| json["filesize"].as_u64()),
        platform: json["extractor_key"].as_str().unwrap_or("Unknown").to_string(),
    })
}

/// Extracts basic video info using --print, which does NOT trigger format selection.
/// Works even when YouTube restricts downloadable formats (e.g. po_token required on datacenter IPs).
async fn try_get_info_nofmt(url: &str, cookies: Option<&str>) -> Option<VideoInfo> {
    let mut args = vec![
        "--no-playlist".to_string(),
        "--quiet".to_string(),
        "--no-warnings".to_string(),
    ];
    if cookies.is_some() {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=web".to_string());
    } else {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=mweb,web".to_string());
    }
    args.push("--extractor-args".to_string());
    args.push("youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".to_string());
    if let Some(path) = cookies {
        args.push("--cookies".to_string());
        args.push(path.to_string());
    }
    if let Some(proxy) = get_proxy() {
        args.push("--proxy".to_string());
        args.push(proxy);
    }
    // Each --print arg outputs one line; order: title, uploader, duration, thumbnail, extractor
    for field in &["%(title)s", "%(uploader)s", "%(duration)s", "%(thumbnail)s", "%(extractor_key)s"] {
        args.push("--print".to_string());
        args.push(field.to_string());
    }
    args.push(url.to_string());

    let output = Command::new(ytdlp_bin())
        .args(&args)
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            log::warn!("yt-dlp nofmt info failed for {}: {}", url, stderr.trim());
        }
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let title = lines.next().filter(|s| !s.is_empty() && *s != "NA")?;
    let uploader = lines.next().unwrap_or("Unknown");
    let duration_str = lines.next().unwrap_or("NA");
    let thumbnail = lines.next().unwrap_or("NA");
    let extractor = lines.next().unwrap_or("Unknown");

    Some(VideoInfo {
        title: title.to_string(),
        uploader: if uploader == "NA" || uploader.is_empty() { "Unknown".to_string() } else { uploader.to_string() },
        duration_seconds: duration_str.parse().ok(),
        thumbnail_url: if thumbnail == "NA" || thumbnail.is_empty() { None } else { Some(thumbnail.to_string()) },
        platform: if extractor == "NA" || extractor.is_empty() { "Unknown".to_string() } else { extractor.to_string() },
        filesize_approx: None,
    })
}

pub async fn extract_with_progress(
    url: String,
    quality: Option<String>,
    audio_only: bool,
    tx: broadcast::Sender<JobEvent>,
    result_store: Arc<Mutex<Option<JobResult>>>,
    cancelled: Arc<AtomicBool>,
) {
    let id = uuid::Uuid::new_v4().to_string();
    let ext = if audio_only { "mp3" } else { "mp4" };
    let tmp_path = format!("/tmp/vidclaw_{}.{}", id, ext);
    let format = if audio_only {
        "bestaudio/best".to_string()
    } else {
        quality_to_format(quality.as_deref())
    };

    let title = get_title(&url).await;
    let filename = build_filename(title.as_deref(), audio_only);

    macro_rules! check_cancelled {
        () => {
            if cancelled.load(Ordering::Relaxed) {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
        };
    }

    // Try without cookies (YouTube, TikTok, Twitter)
    match run_with_progress(&url, &tmp_path, &format, audio_only, CookieSource::None, &tx, &cancelled).await {
        Ok(()) => {
            *result_store.lock().await = Some(JobResult { file_path: tmp_path, filename: filename.clone() });
            let _ = tx.send(JobEvent::Done { filename });
            return;
        }
        Err(ref e) if e == "Cancelled" => {
            let _ = tokio::fs::remove_file(&tmp_path).await;
            let _ = tx.send(JobEvent::Cancelled);
            return;
        }
        Err(ref stderr) => {
            if let Some(msg) = hard_error_message(stderr) {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                let _ = tx.send(JobEvent::Error { message: msg });
                return;
            }
        }
    }

    check_cancelled!();

    // Try session cookies from env vars (INSTAGRAM_SESSION_ID / FACEBOOK_SESSION_COOKIES)
    if let Some(session_file) = build_session_cookies(&url).await {
        let _ = tx.send(JobEvent::Authenticating { method: "session token".into() });
        let _ = tokio::fs::remove_file(&tmp_path).await;
        let result = run_with_progress(&url, &tmp_path, &format, audio_only, CookieSource::File(&session_file), &tx, &cancelled).await;
        let _ = tokio::fs::remove_file(&session_file).await; // always clean up temp cookies
        match result {
            Ok(()) => {
                *result_store.lock().await = Some(JobResult { file_path: tmp_path, filename: filename.clone() });
                let _ = tx.send(JobEvent::Done { filename });
                return;
            }
            Err(ref e) if e == "Cancelled" => {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
            Err(_) => {}
        }
    }

    check_cancelled!();

    // Try cookies file
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        let _ = tx.send(JobEvent::Authenticating { method: "cookies file".into() });
        let _ = tokio::fs::remove_file(&tmp_path).await;
        match run_with_progress(&url, &tmp_path, &format, audio_only, CookieSource::File(&cookies_path), &tx, &cancelled).await {
            Ok(()) => {
                *result_store.lock().await = Some(JobResult { file_path: tmp_path, filename: filename.clone() });
                let _ = tx.send(JobEvent::Done { filename });
                return;
            }
            Err(ref e) if e == "Cancelled" => {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
            Err(_) => {}
        }
    }

    // Try browser cookies
    let mut last_stderr = String::new();
    for browser in BROWSERS {
        check_cancelled!();
        let _ = tx.send(JobEvent::Authenticating { method: browser.to_string() });
        let _ = tokio::fs::remove_file(&tmp_path).await;
        match run_with_progress(&url, &tmp_path, &format, audio_only, CookieSource::Browser(browser), &tx, &cancelled).await {
            Ok(()) => {
                *result_store.lock().await = Some(JobResult { file_path: tmp_path, filename: filename.clone() });
                let _ = tx.send(JobEvent::Done { filename });
                return;
            }
            Err(ref e) if e == "Cancelled" => {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
            Err(stderr) => {
                if !stderr.is_empty() && !stderr.contains("could not find") {
                    last_stderr = stderr;
                }
            }
        }
    }

    let _ = tokio::fs::remove_file(&tmp_path).await;
    let _ = tx.send(JobEvent::Error { message: auth_error_message(&last_stderr) });
}

enum CookieSource<'a> {
    None,
    File(&'a str),
    Browser(&'a str),
}

async fn run_with_progress(
    url: &str,
    tmp_path: &str,
    format: &str,
    audio_only: bool,
    cookies: CookieSource<'_>,
    tx: &broadcast::Sender<JobEvent>,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), String> {
    let mut args = build_args(tmp_path, format, audio_only, &cookies);
    args.push(url.into());

    let mut child = Command::new(ytdlp_bin())
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let tx_clone = tx.clone();
    let stdout_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if let Some((percent, speed, eta)) = parse_progress_line(&line) {
                let _ = tx_clone.send(JobEvent::Progress { percent, speed, eta });
            } else if line.contains("[Merger]") || line.contains("Merging formats") {
                let _ = tx_clone.send(JobEvent::Merging);
            }
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut lines = BufReader::new(stderr).lines();
        let mut errors = Vec::new();
        while let Ok(Some(line)) = lines.next_line().await {
            if !line.trim().is_empty() { errors.push(line); }
        }
        errors.join("\n")
    });

    let cancel_flag = cancelled.clone();
    let status = tokio::select! {
        s = child.wait() => s.map(|s| s.success()).unwrap_or(false),
        _ = async move {
            loop {
                if cancel_flag.load(Ordering::Relaxed) { return; }
                tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
            }
        } => {
            let _ = child.kill().await;
            stdout_task.abort();
            stderr_task.abort();
            return Err("Cancelled".into());
        }
    };

    let _ = stdout_task.await;
    let stderr_out = stderr_task.await.unwrap_or_default();

    if status {
        if let Ok(meta) = tokio::fs::metadata(tmp_path).await {
            if meta.len() > 0 { return Ok(()); }
        }
    }
    Err(stderr_out)
}

fn build_args(tmp_path: &str, format: &str, audio_only: bool, cookies: &CookieSource<'_>) -> Vec<String> {
    let extractor_args = match cookies {
        CookieSource::None => "youtube:player_client=mweb,web",
        _ => "youtube:player_client=web",
    };
    let mut args: Vec<String> = vec![
        "-o".into(), tmp_path.into(),
        "-f".into(), format.into(),
        "--no-playlist".into(),
        "--newline".into(),
        "--extractor-args".into(),
        extractor_args.into(),
        "--extractor-args".into(),
        "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".into(),
    ];
    if audio_only {
        args.push("--extract-audio".into());
        args.push("--audio-format".into());
        args.push("mp3".into());
        args.push("--audio-quality".into());
        args.push("0".into()); // best quality
    } else {
        args.push("--merge-output-format".into());
        args.push("mp4".into());
    }

    match cookies {
        CookieSource::None => {}
        CookieSource::File(path) => { args.push("--cookies".into()); args.push(path.to_string()); }
        CookieSource::Browser(b) => { args.push("--cookies-from-browser".into()); args.push(b.to_string()); }
    }

    if let Some(proxy) = get_proxy() {
        args.push("--proxy".into());
        args.push(proxy);
    }

    args
}

fn get_proxy() -> Option<String> {
    std::env::var("YTDLP_PROXY").ok().filter(|s| !s.is_empty())
}

/// Resolve yt-dlp binary path. Priority:
/// 1. YTDLP_PATH env var (explicit override)
/// 2. Venv at YTDLP_VENV or ~/.local/share/vidclaw/venv (has curl-cffi for TikTok)
/// 3. System yt-dlp
fn ytdlp_bin() -> String {
    if let Ok(path) = std::env::var("YTDLP_PATH") {
        if !path.is_empty() { return path; }
    }

    let venv_dir = std::env::var("YTDLP_VENV")
        .unwrap_or_else(|_| expand_home("~/.local/share/vidclaw/venv"));
    let venv_bin = format!("{}/bin/yt-dlp", venv_dir);

    if std::path::Path::new(&venv_bin).exists() {
        return venv_bin;
    }

    "yt-dlp".to_string()
}

fn parse_progress_line(line: &str) -> Option<(f32, Option<String>, Option<String>)> {
    if !line.contains("[download]") { return None; }
    let after = line.split("[download]").nth(1)?.trim();
    let percent_str = after.split('%').next()?.trim();
    let percent: f32 = percent_str.parse().ok()?;
    let speed = extract_between(line, " at ", "/s")
        .filter(|s| s != "Unknown")
        .map(|s| format!("{}/s", s));
    let eta = line.find("ETA ")
        .and_then(|pos| line[pos + 4..].split_whitespace().next().map(str::to_string))
        .filter(|s| s != "Unknown");
    Some((percent, speed, eta))
}

fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let start_pos = s.find(start)? + start.len();
    let after = s[start_pos..].trim();
    let end_pos = after.find(end)?;
    Some(after[..end_pos].trim().to_string())
}

fn quality_to_format(quality: Option<&str>) -> String {
    match quality.unwrap_or("best") {
        "1080p" => "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=1080]+bestaudio/best[height<=1080]/bestvideo+bestaudio/best".into(),
        "720p"  => "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=720]+bestaudio/best[height<=720]/bestvideo+bestaudio/best".into(),
        "480p"  => "bestvideo[height<=480][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=480]+bestaudio/best[height<=480]/worstvideo+worstaudio/worst".into(),
        "360p"  => "bestvideo[height<=360][ext=mp4]+bestaudio[ext=m4a]/bestvideo[height<=360]+bestaudio/best[height<=360]/worstvideo+worstaudio/worst".into(),
        _       => "bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best[ext=mp4]/best".into(),
    }
}

fn hard_error_message(stderr: &str) -> Option<String> {
    let s = stderr.to_lowercase();
    if s.contains("no video could be found") || s.contains("no video in this") {
        Some("No video found in this post. It may be a text or image-only post.".into())
    } else if s.contains("video unavailable") || s.contains("has been removed") || s.contains("no longer available") {
        Some("This video is no longer available.".into())
    } else if s.contains("private video") || s.contains("private post") {
        Some("This video is private.".into())
    } else if s.contains("age-restricted") || s.contains("age restricted") {
        Some("Age-restricted content. Log into the platform in your browser and retry.".into())
    } else if s.contains("not available in your country") {
        Some("This video is geo-restricted and not available in your region.".into())
    } else if s.contains("copyright") {
        Some("This video is unavailable due to a copyright claim.".into())
    } else {
        None
    }
}

fn auth_error_message(stderr: &str) -> String {
    let s = stderr.to_lowercase();
    if s.contains("rate") || s.contains("too many requests") || s.contains("429") {
        "Rate limited by the platform. Wait a few minutes and try again.".into()
    } else if s.contains("login") || s.contains("sign in") || s.contains("authentication") || s.contains("empty media response") {
        "This video requires authentication. Log into the platform in Chromium or Brave, \
         or upload your cookies.txt file.".into()
    } else {
        "Could not download this video. It may be private, geo-restricted, or require authentication.".into()
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

fn build_filename(title: Option<&str>, audio_only: bool) -> String {
    let ext = if audio_only { "mp3" } else { "mp4" };
    match title {
        Some(t) => {
            let safe: String = t.chars()
                .map(|c| match c {
                    '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                    c => c,
                })
                .take(80)
                .collect();
            format!("{}.{}", safe, ext)
        }
        None => format!("video.{}", ext),
    }
}

async fn get_title(url: &str) -> Option<String> {
    let output = Command::new(ytdlp_bin())
        .args(["--get-title", "--no-playlist", "--quiet", "--no-warnings", url])
        .output()
        .await
        .ok()?;
    if output.status.success() {
        let t = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !t.is_empty() { return Some(t); }
    }
    None
}
