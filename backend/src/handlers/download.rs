use actix_web::{http::header, web, HttpResponse};
use tokio::io::AsyncReadExt;

use crate::api::ytdlp;
use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::DownloadRequest;
use crate::util;

pub async fn download_video(
    _config: web::Data<Config>,
    req: web::Json<DownloadRequest>,
) -> AppResult<HttpResponse> {
    let url = req.url.trim();
    util::validate_url(url)?;

    log::info!("Download request: {}", url);

    let extracted = ytdlp::extract(url).await.map_err(|e| {
        log::warn!("yt-dlp failed for {}: {:?}", req.url, e);
        e
    })?;

    let file_path = extracted.file_path.clone();
    let filename = extracted.filename.clone();

    log::info!("Serving {} ({} bytes)", filename, {
        tokio::fs::metadata(&file_path)
            .await
            .map(|m| m.len())
            .unwrap_or(0)
    });

    // Read file and stream to client in chunks
    let file_size = tokio::fs::metadata(&file_path)
        .await
        .map(|m| m.len())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut file = tokio::fs::File::open(&file_path)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut body = Vec::with_capacity(file_size as usize);
    file.read_to_end(&mut body)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Delete temp file after reading into memory
    let _ = tokio::fs::remove_file(&file_path).await;

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "video/mp4"))
        .append_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ))
        .append_header((header::CONTENT_LENGTH, body.len().to_string()))
        .body(body))
}
