use actix_web::{error::ErrorBadGateway, http::header, web, HttpResponse};
use futures::stream::StreamExt;
use reqwest::Client;

use crate::api::get_adapter_for_platform;
use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::DownloadRequest;
use crate::util;

pub async fn download_video(
    _config: web::Data<Config>,
    req: web::Json<DownloadRequest>,
) -> AppResult<HttpResponse> {
    util::validate_url(&req.url)?;

    let platform = util::detect_platform(&req.url);
    let video_id = util::extract_video_id(&req.url, platform)?;

    log::info!("Download request for {} video: {}", platform.as_str(), video_id);

    let adapter = get_adapter_for_platform(platform)?;
    if !adapter.validate_url(&req.url).await? {
        return Err(AppError::InvalidUrl(format!(
            "URL is not a valid {} link",
            platform.as_str()
        )));
    }

    let download_url = adapter.get_download_url(&req.url).await?;

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut request_builder = client
        .get(&download_url)
        .header("Accept", "video/*,*/*");

    if platform == crate::models::Platform::TikTok {
        request_builder = request_builder.header(header::REFERER, &req.url);
    }

    let upstream_response = request_builder
        .send()
        .await
        .map_err(|e| AppError::NetworkError(e.to_string()))?;

    if !upstream_response.status().is_success() {
        return Err(AppError::PlatformError(format!(
            "Failed to fetch media from {}: {}",
            platform.as_str(),
            upstream_response.status()
        )));
    }

    let filename = util::generate_filename(&platform, &video_id);
    let content_length = upstream_response.content_length();
    let content_type = upstream_response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let stream = upstream_response
        .bytes_stream()
        .map(|chunk| chunk.map_err(|e| ErrorBadGateway(e)));

    let mut response = HttpResponse::Ok();
    response.append_header((header::CONTENT_TYPE, content_type));
    response.append_header((header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename)));

    if let Some(length) = content_length {
        response.append_header((header::CONTENT_LENGTH, length.to_string()));
    }

    Ok(response.streaming(stream))
}
