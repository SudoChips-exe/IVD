use actix_web::{web, HttpResponse};
use serde::Serialize;
use uuid::Uuid;

use crate::api::ytdlp;
use crate::config::Config;
use crate::error::AppResult;
use crate::jobs::JobStore;
use crate::models::DownloadRequest;
use crate::util;

#[derive(Serialize)]
struct StartResponse {
    job_id: String,
}

pub async fn download_video(
    _config: web::Data<Config>,
    jobs: web::Data<JobStore>,
    req: web::Json<DownloadRequest>,
) -> AppResult<HttpResponse> {
    let url = req.url.trim().to_string();
    util::validate_url(&url)?;

    let quality = req.quality.clone();
    let audio_only = req.audio_only.unwrap_or(false);
    log::info!(
        "Download job started: {} (quality: {}, audio_only: {})",
        url,
        quality.as_deref().unwrap_or("best"),
        audio_only
    );

    let job_id = Uuid::new_v4().to_string();
    let (tx, result_store, cancelled) = jobs.create(&job_id).await;

    tokio::spawn(async move {
        ytdlp::extract_with_progress(url, quality, audio_only, tx, result_store, cancelled).await;
    });

    Ok(HttpResponse::Ok().json(StartResponse { job_id }))
}
