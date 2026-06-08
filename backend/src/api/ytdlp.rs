use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::{broadcast, Mutex};

use crate::error::{AppError, AppResult};
use crate::jobs::{JobEvent, JobResult};
use crate::models::{PlaylistEntry, PlaylistInfo, VideoInfo};

const BROWSERS: &[&str] = &["chrome", "chromium", "firefox", "brave", "edge"];
const COOKIES_FILE: &str = "~/.config/vidclaw/cookies.txt";

async fn build_session_cookies(url: &str) -> Option<String> {
    let is_instagram = url.contains("instagram.com") || url.contains("ig.me");
    let is_facebook  = url.contains("facebook.com")  || url.contains("fb.watch");
    if !is_instagram && !is_facebook { return None; }

    let mut lines = vec!["# Netscape HTTP Cookie File".to_string()];
    let mut has_cookies = false;

    if is_instagram {
        if let Ok(sid) = std::env::var("INSTAGRAM_SESSION_ID") {
            if !sid.is_empty() {
                lines.push(format!(".instagram.com\tTRUE\t/\tTRUE\t9999999999\tsessionid\t{}", sid));
                has_cookies = true;
            }
        }
    }
    if is_facebook {
        if let Ok(fb) = std::env::var("FACEBOOK_SESSION_COOKIES") {
            for pair in fb.split(';') {
                if let Some((k, v)) = pair.trim().split_once('=') {
                    lines.push(format!(".facebook.com\tTRUE\t/\tTRUE\t9999999999\t{}\t{}", k.trim(), v.trim()));
                    has_cookies = true;
                }
            }
        }
    }
    if !has_cookies { return None; }

    let tmp = format!("/tmp/vidclaw_cookies_{}.txt", uuid::Uuid::new_v4());
    tokio::fs::write(&tmp, lines.join("\n") + "\n").await.ok()?;
    Some(tmp)
}

pub async fn get_info(url: &str) -> AppResult<VideoInfo> {
    if get_proxy().is_none() {
        if let Some(info) = try_get_info(url, None, true).await { return Ok(info); }
    }
    if let Some(info) = try_get_info(url, None, false).await { return Ok(info); }
    if let Some(session_file) = build_session_cookies(url).await {
        let result = try_get_info(url, Some(&session_file), false).await;
        let _ = tokio::fs::remove_file(&session_file).await;
        if let Some(info) = result { return Ok(info); }
    }
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        if let Some(info) = try_get_info(url, Some(&cookies_path), false).await { return Ok(info); }
        if let Some(info) = try_get_info_nofmt(url, Some(&cookies_path), false).await { return Ok(info); }
    }
    if let Some(info) = try_get_info_nofmt(url, None, false).await { return Ok(info); }
    Err(AppError::PlatformError("Could not fetch video info. Check the URL or platform support.".to_string()))
}

async fn try_get_info(url: &str, cookies: Option<&str>, force_ipv6: bool) -> Option<VideoInfo> {
    let mut args = vec![
        "--dump-json".to_string(), "--no-playlist".to_string(),
        "--quiet".to_string(), "--no-warnings".to_string(),
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
    if let Some(path) = cookies { args.push("--cookies".to_string()); args.push(path.to_string()); }
    if let Some(proxy) = get_proxy() { args.push("--proxy".to_string()); args.push(proxy); }
    else if force_ipv6 { args.push("--force-ipv6".to_string()); }
    args.push(url.to_string());

    let output = Command::new(ytdlp_bin()).args(&args).output().await.ok()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() { log::warn!("yt-dlp info failed for {}: {}", url, stderr.trim()); }
        let sl = stderr.to_lowercase();
        if sl.contains("there is no video in this post")
            || sl.contains("no video formats found")
            || sl.contains("no video could be found")
        {
            return Some(VideoInfo {
                title: "Post".to_string(),
                uploader: "Unknown".to_string(),
                duration_seconds: None,
                thumbnail_url: None,
                filesize_approx: None,
                platform: platform_from_url(url),
                is_image: true,
            });
        }
        return None;
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let thumbnail = json["thumbnail"].as_str()
        .or_else(|| json["thumbnails"].as_array()?.last()?.get("url")?.as_str())
        .map(str::to_string);

    let ext = json["ext"].as_str().unwrap_or("").to_lowercase();
    let vcodec = json["vcodec"].as_str().unwrap_or("").to_lowercase();
    let is_image = matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif")
        || (vcodec == "none" && !matches!(ext.as_str(), "mp3" | "aac" | "m4a" | "ogg" | "opus" | "webm" | "mp4" | ""));

    Some(VideoInfo {
        title: json["title"].as_str().unwrap_or("Unknown").to_string(),
        uploader: json["uploader"].as_str()
            .or_else(|| json["channel"].as_str())
            .or_else(|| json["creator"].as_str())
            .unwrap_or("Unknown").to_string(),
        duration_seconds: json["duration"].as_u64(),
        thumbnail_url: thumbnail,
        filesize_approx: json["filesize_approx"].as_u64().or_else(|| json["filesize"].as_u64()),
        platform: json["extractor_key"].as_str().unwrap_or("Unknown").to_string(),
        is_image,
    })
}

async fn try_get_info_nofmt(url: &str, cookies: Option<&str>, force_ipv6: bool) -> Option<VideoInfo> {
    let mut args = vec!["--no-playlist".to_string(), "--quiet".to_string(), "--no-warnings".to_string()];
    if cookies.is_some() {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=web".to_string());
    } else {
        args.push("--extractor-args".to_string());
        args.push("youtube:player_client=mweb,web".to_string());
    }
    args.push("--extractor-args".to_string());
    args.push("youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".to_string());
    if let Some(path) = cookies { args.push("--cookies".to_string()); args.push(path.to_string()); }
    if let Some(proxy) = get_proxy() { args.push("--proxy".to_string()); args.push(proxy); }
    else if force_ipv6 { args.push("--force-ipv6".to_string()); }
    for field in &["%(title)s", "%(uploader)s", "%(duration)s", "%(thumbnail)s", "%(extractor_key)s"] {
        args.push("--print".to_string()); args.push(field.to_string());
    }
    args.push(url.to_string());

    let output = Command::new(ytdlp_bin()).args(&args).output().await.ok()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() { log::warn!("yt-dlp nofmt info failed for {}: {}", url, stderr.trim()); }
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let title     = lines.next().filter(|s| !s.is_empty() && *s != "NA")?;
    let uploader  = lines.next().unwrap_or("Unknown");
    let dur_str   = lines.next().unwrap_or("NA");
    let thumbnail = lines.next().unwrap_or("NA");
    let extractor = lines.next().unwrap_or("Unknown");

    Some(VideoInfo {
        title: title.to_string(),
        uploader: if uploader == "NA" || uploader.is_empty() { "Unknown".to_string() } else { uploader.to_string() },
        duration_seconds: dur_str.parse().ok(),
        thumbnail_url: if thumbnail == "NA" || thumbnail.is_empty() { None } else { Some(thumbnail.to_string()) },
        platform: if extractor == "NA" || extractor.is_empty() { "Unknown".to_string() } else { extractor.to_string() },
        filesize_approx: None,
        is_image: false,
    })
}

pub async fn get_playlist_info(url: &str) -> AppResult<PlaylistInfo> {
    let mut args = vec![
        "--flat-playlist".to_string(), "--dump-json".to_string(),
        "--quiet".to_string(), "--no-warnings".to_string(),
        "--playlist-end".to_string(), "50".to_string(),
        "--extractor-args".to_string(), "youtube:player_client=mweb,web".to_string(),
        "--extractor-args".to_string(), "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".to_string(),
    ];
    let cookies_path = expand_home(COOKIES_FILE);
    if tokio::fs::metadata(&cookies_path).await.is_ok() {
        args.push("--cookies".to_string());
        args.push(cookies_path);
    }
    if let Some(proxy) = get_proxy() { args.push("--proxy".to_string()); args.push(proxy); }
    args.push(url.to_string());

    let output = Command::new(ytdlp_bin()).args(&args).output().await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    let mut playlist_title = "Playlist".to_string();

    for line in stdout.lines() {
        let Ok(json) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        if json["_type"].as_str() == Some("playlist") {
            if let Some(t) = json["title"].as_str() { playlist_title = t.to_string(); }
            continue;
        }
        let entry_url = json["url"].as_str()
            .or_else(|| json["webpage_url"].as_str())
            .map(str::to_string)
            .unwrap_or_default();
        if entry_url.is_empty() { continue; }
        entries.push(PlaylistEntry {
            url: entry_url,
            title: json["title"].as_str().unwrap_or("Unknown").to_string(),
            thumbnail: json["thumbnail"].as_str().map(str::to_string),
            duration_seconds: json["duration"].as_u64(),
        });
    }

    if entries.is_empty() {
        return Err(AppError::PlatformError("No videos found in this playlist.".to_string()));
    }

    let total = entries.len();
    Ok(PlaylistInfo { title: playlist_title, entries, total })
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
    let tmp_dir = format!("/tmp/vidclaw_{}", id);
    tokio::fs::create_dir_all(&tmp_dir).await.ok();

    let video_format = if audio_only {
        "bestaudio/best".to_string()
    } else {
        quality_to_format(quality.as_deref())
    };

    let title = get_title(&url).await;
    let cookies_path = expand_home(COOKIES_FILE);
    let has_cookies = tokio::fs::metadata(&cookies_path).await.is_ok();

    macro_rules! reset_dir {
        () => {{
            let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
            tokio::fs::create_dir_all(&tmp_dir).await.ok();
        }};
    }

    macro_rules! check_cancelled {
        () => {
            if cancelled.load(Ordering::Relaxed) {
                let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
        };
    }

    macro_rules! handle {
        ($result:expr, hard_fail) => {
            match $result {
                Ok(file_path) => {
                    let ct = content_type_from_path(&file_path);
                    let fname = build_filename(title.as_deref(), audio_only, &ct);
                    *result_store.lock().await = Some(JobResult { file_path, filename: fname.clone(), content_type: ct });
                    let _ = tx.send(JobEvent::Done { filename: fname });
                    return;
                }
                Err(ref e) if e == "Cancelled" => {
                    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                    let _ = tx.send(JobEvent::Cancelled);
                    return;
                }
                Err(ref s) => {
                    if let Some(m) = hard_error_message(s) {
                        let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                        let _ = tx.send(JobEvent::Error { message: m });
                        return;
                    }
                }
            }
        };
        ($result:expr, soft_fail) => {
            match $result {
                Ok(file_path) => {
                    let ct = content_type_from_path(&file_path);
                    let fname = build_filename(title.as_deref(), audio_only, &ct);
                    *result_store.lock().await = Some(JobResult { file_path, filename: fname.clone(), content_type: ct });
                    let _ = tx.send(JobEvent::Done { filename: fname });
                    return;
                }
                Err(ref e) if e == "Cancelled" => {
                    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                    let _ = tx.send(JobEvent::Cancelled);
                    return;
                }
                Err(_) => {}
            }
        };
    }

    // 1. IPv6, no cookies
    if get_proxy().is_none() {
        reset_dir!();
        handle!(run_with_progress(&url, &tmp_dir, &video_format, audio_only, false, CookieSource::None, true, &tx, &cancelled).await, hard_fail);
        check_cancelled!();
    }

    // 2. IPv4, no cookies
    reset_dir!();
    handle!(run_with_progress(&url, &tmp_dir, &video_format, audio_only, false, CookieSource::None, false, &tx, &cancelled).await, hard_fail);
    check_cancelled!();

    // 3. Session cookies (Instagram / Facebook env vars)
    if let Some(session_file) = build_session_cookies(&url).await {
        let _ = tx.send(JobEvent::Authenticating { method: "session token".into() });
        reset_dir!();
        let result = run_with_progress(&url, &tmp_dir, &video_format, audio_only, false, CookieSource::File(&session_file), false, &tx, &cancelled).await;
        let _ = tokio::fs::remove_file(&session_file).await;
        handle!(result, soft_fail);
        check_cancelled!();
    }

    // 4. Cookies file
    if has_cookies {
        let _ = tx.send(JobEvent::Authenticating { method: "cookies file".into() });
        reset_dir!();
        handle!(run_with_progress(&url, &tmp_dir, &video_format, audio_only, false, CookieSource::File(&cookies_path), false, &tx, &cancelled).await, soft_fail);
        check_cancelled!();
    }

    // 5. Browser cookies
    let mut last_stderr = String::new();
    for browser in BROWSERS {
        check_cancelled!();
        let _ = tx.send(JobEvent::Authenticating { method: browser.to_string() });
        reset_dir!();
        match run_with_progress(&url, &tmp_dir, &video_format, audio_only, false, CookieSource::Browser(browser), false, &tx, &cancelled).await {
            Ok(file_path) => {
                let ct = content_type_from_path(&file_path);
                let fname = build_filename(title.as_deref(), audio_only, &ct);
                *result_store.lock().await = Some(JobResult { file_path, filename: fname.clone(), content_type: ct });
                let _ = tx.send(JobEvent::Done { filename: fname });
                return;
            }
            Err(ref e) if e == "Cancelled" => {
                let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                let _ = tx.send(JobEvent::Cancelled);
                return;
            }
            Err(s) => { if !s.is_empty() && !s.contains("could not find") { last_stderr = s; } }
        }
    }

    // 6. Image/post fallback — handles photo posts where video format returns no results
    if !audio_only {
        check_cancelled!();
        reset_dir!();
        handle!(run_with_progress(&url, &tmp_dir, "best", false, true, CookieSource::None, false, &tx, &cancelled).await, soft_fail);
        check_cancelled!();

        // Try session cookies in image mode (covers Instagram/Facebook image posts requiring auth)
        if let Some(session_file) = build_session_cookies(&url).await {
            reset_dir!();
            let result = run_with_progress(&url, &tmp_dir, "best", false, true, CookieSource::File(&session_file), false, &tx, &cancelled).await;
            let _ = tokio::fs::remove_file(&session_file).await;
            handle!(result, soft_fail);
            check_cancelled!();
        }

        if has_cookies {
            reset_dir!();
            match run_with_progress(&url, &tmp_dir, "best", false, true, CookieSource::File(&cookies_path), false, &tx, &cancelled).await {
                Ok(file_path) => {
                    let ct = content_type_from_path(&file_path);
                    let fname = build_filename(title.as_deref(), false, &ct);
                    *result_store.lock().await = Some(JobResult { file_path, filename: fname.clone(), content_type: ct });
                    let _ = tx.send(JobEvent::Done { filename: fname });
                    return;
                }
                Err(ref e) if e == "Cancelled" => {
                    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
                    let _ = tx.send(JobEvent::Cancelled);
                    return;
                }
                Err(s) => { if !s.is_empty() { last_stderr = s; } }
            }
        }
    }

    let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
    let _ = tx.send(JobEvent::Error { message: auth_error_message(&last_stderr) });
}

enum CookieSource<'a> {
    None,
    File(&'a str),
    Browser(&'a str),
}

async fn run_with_progress(
    url: &str,
    tmp_dir: &str,
    format: &str,
    audio_only: bool,
    image_mode: bool,
    cookies: CookieSource<'_>,
    force_ipv6: bool,
    tx: &broadcast::Sender<JobEvent>,
    cancelled: &Arc<AtomicBool>,
) -> Result<String, String> {
    let tmp_template = format!("{}/media.%(ext)s", tmp_dir);
    let mut args = build_args(&tmp_template, format, audio_only, image_mode, &cookies, force_ipv6);
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
        if let Some(path) = find_output_file(tmp_dir).await {
            return Ok(path);
        }
    }
    Err(stderr_out)
}

async fn find_output_file(dir: &str) -> Option<String> {
    let mut entries = tokio::fs::read_dir(dir).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_file() {
            if let Ok(meta) = tokio::fs::metadata(&path).await {
                if meta.len() > 0 {
                    return path.to_str().map(str::to_string);
                }
            }
        }
    }
    None
}

fn content_type_from_path(path: &str) -> String {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "mp4" | "m4v"  => "video/mp4",
        "webm"         => "video/webm",
        "mkv"          => "video/x-matroska",
        "mp3"          => "audio/mpeg",
        "m4a"          => "audio/mp4",
        "ogg"          => "audio/ogg",
        "jpg" | "jpeg" => "image/jpeg",
        "png"          => "image/png",
        "webp"         => "image/webp",
        "gif"          => "image/gif",
        _              => "application/octet-stream",
    }.to_string()
}

fn build_args(tmp_template: &str, format: &str, audio_only: bool, image_mode: bool, cookies: &CookieSource<'_>, force_ipv6: bool) -> Vec<String> {
    let extractor_args = match cookies {
        CookieSource::None => "youtube:player_client=mweb,web",
        _ => "youtube:player_client=web",
    };
    let mut args: Vec<String> = vec![
        "-o".into(), tmp_template.into(),
        "-f".into(), format.into(),
        "--no-playlist".into(),
        "--newline".into(),
        "--extractor-args".into(), extractor_args.into(),
        "--extractor-args".into(), "youtubepot-bgutilscript:server_home=/opt/bgutil-pot/server".into(),
    ];
    if audio_only {
        args.push("--extract-audio".into());
        args.push("--audio-format".into());
        args.push("mp3".into());
        args.push("--audio-quality".into());
        args.push("0".into());
    } else if !image_mode {
        args.push("--merge-output-format".into());
        args.push("mp4".into());
    }
    match cookies {
        CookieSource::None => {}
        CookieSource::File(p) => { args.push("--cookies".into()); args.push(p.to_string()); }
        CookieSource::Browser(b) => { args.push("--cookies-from-browser".into()); args.push(b.to_string()); }
    }
    if let Some(proxy) = get_proxy() { args.push("--proxy".into()); args.push(proxy); }
    else if force_ipv6 { args.push("--force-ipv6".into()); }
    args
}

fn get_proxy() -> Option<String> {
    std::env::var("YTDLP_PROXY").ok().filter(|s| !s.is_empty())
}

fn ytdlp_bin() -> String {
    if let Ok(path) = std::env::var("YTDLP_PATH") {
        if !path.is_empty() { return path; }
    }
    let venv_dir = std::env::var("YTDLP_VENV")
        .unwrap_or_else(|_| expand_home("~/.local/share/vidclaw/venv"));
    let venv_bin = format!("{}/bin/yt-dlp", venv_dir);
    if std::path::Path::new(&venv_bin).exists() { return venv_bin; }
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
    if s.contains("video unavailable") || s.contains("has been removed") || s.contains("no longer available") {
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
        "This content requires authentication. Log into the platform in Chromium or Brave, or upload your cookies.txt file.".into()
    } else if s.contains("no video") || s.contains("no media") {
        "No downloadable media found. This post may be text-only or unsupported.".into()
    } else {
        "Could not download this content. It may be private, geo-restricted, or require authentication.".into()
    }
}

fn platform_from_url(url: &str) -> String {
    let u = url.to_lowercase();
    if u.contains("instagram.com") || u.contains("ig.me") { "Instagram".into() }
    else if u.contains("tiktok.com") { "TikTok".into() }
    else if u.contains("twitter.com") || u.contains("x.com") { "Twitter".into() }
    else if u.contains("facebook.com") || u.contains("fb.watch") { "Facebook".into() }
    else if u.contains("youtube.com") || u.contains("youtu.be") { "YouTube".into() }
    else { "Unknown".into() }
}

fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") { return format!("{}{}", home, &path[1..]); }
    }
    path.to_string()
}

fn build_filename(title: Option<&str>, audio_only: bool, content_type: &str) -> String {
    let ext = if audio_only {
        "mp3"
    } else {
        match content_type {
            "video/mp4" | "video/x-matroska" => "mp4",
            "video/webm" => "webm",
            "audio/mpeg" => "mp3",
            "audio/mp4"  => "m4a",
            "image/jpeg" => "jpg",
            "image/png"  => "png",
            "image/webp" => "webp",
            "image/gif"  => "gif",
            _ => "mp4",
        }
    };
    match title {
        Some(t) => {
            let safe: String = t.chars()
                .map(|c| match c { '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_', c => c })
                .take(80)
                .collect();
            format!("{}.{}", safe, ext)
        }
        None => format!("download.{}", ext),
    }
}

async fn get_title(url: &str) -> Option<String> {
    let output = Command::new(ytdlp_bin())
        .args(["--get-title", "--no-playlist", "--quiet", "--no-warnings", url])
        .output().await.ok()?;
    if output.status.success() {
        let t = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !t.is_empty() { return Some(t); }
    }
    None
}
