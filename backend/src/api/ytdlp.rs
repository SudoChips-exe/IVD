use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, Mutex};

use crate::jobs::{JobEvent, JobResult};

const BROWSERS: &[&str] = &["chrome", "chromium", "firefox", "brave", "edge"];
const COOKIES_FILE: &str = "~/.config/vidclaw/cookies.txt";

pub async fn extract_with_progress(
    url: String,
    quality: Option<String>,
    tx: broadcast::Sender<JobEvent>,
    result_store: Arc<Mutex<Option<JobResult>>>,
    cancelled: Arc<AtomicBool>,
) {
    let id = uuid::Uuid::new_v4().to_string();
    let tmp_path = format!("/tmp/vidclaw_{}.mp4", id);
    let format = quality_to_format(quality.as_deref());

    let title = get_title(&url).await;
    let filename = build_filename(title.as_deref());

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
    match run_with_progress(&url, &tmp_path, &format, CookieSource::None, &tx, &cancelled).await {
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

    // Try cookies file
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        let _ = tx.send(JobEvent::Authenticating { method: "cookies file".into() });
        let _ = tokio::fs::remove_file(&tmp_path).await;
        match run_with_progress(&url, &tmp_path, &format, CookieSource::File(&cookies_path), &tx, &cancelled).await {
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
        match run_with_progress(&url, &tmp_path, &format, CookieSource::Browser(browser), &tx, &cancelled).await {
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
    cookies: CookieSource<'_>,
    tx: &broadcast::Sender<JobEvent>,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), String> {
    let mut args = build_args(tmp_path, format, &cookies);
    args.push(url.into());

    let mut child = Command::new("yt-dlp")
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

fn build_args(tmp_path: &str, format: &str, cookies: &CookieSource<'_>) -> Vec<String> {
    let mut args: Vec<String> = vec![
        "-o".into(), tmp_path.into(),
        "-f".into(), format.into(),
        "--merge-output-format".into(), "mp4".into(),
        "--no-playlist".into(),
        "--newline".into(),
    ];

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

fn build_filename(title: Option<&str>) -> String {
    match title {
        Some(t) => {
            let safe: String = t.chars()
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
        if !t.is_empty() { return Some(t); }
    }
    None
}
