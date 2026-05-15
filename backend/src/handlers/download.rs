use actix_web::{web, HttpResponse};
use crate::config::Config;
use crate::error::AppResult;
use crate::models::{DownloadRequest, VideoMetadata};
use crate::util;

pub async fn download_video(
    config: web::Data<Config>,
    req: web::Json<DownloadRequest>,
) -> AppResult<HttpResponse> {
    // Validate URL
    util::validate_url(&req.url)?;

    // Detect platform
    let platform = util::detect_platform(&req.url);

    // Extract video ID
    let video_id = util::extract_video_id(&req.url, platform)?;

    log::info!("Download request for {} video: {}", platform.as_str(), video_id);

    // TODO: Route to appropriate platform adapter
    // For now, return placeholder response
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "processing",
        "platform": platform.as_str(),
        "video_id": video_id,
        "message": "Platform integration coming soon"
    })))
}
