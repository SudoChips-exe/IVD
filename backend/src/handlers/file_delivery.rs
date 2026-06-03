use actix_web::{http::header, web, HttpResponse};
use tokio::io::AsyncReadExt;

use crate::error::{AppError, AppResult};
use crate::jobs::JobStore;

pub async fn download_file(
    job_id: web::Path<String>,
    jobs: web::Data<JobStore>,
) -> AppResult<HttpResponse> {
    let result = jobs
        .take_result(&job_id)
        .await
        .ok_or_else(|| AppError::VideoNotFound("Job not found or file not ready".to_string()))?;

    let file_path = &result.file_path;
    let filename = &result.filename;

    let file_size = tokio::fs::metadata(file_path)
        .await
        .map(|m| m.len())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut file = tokio::fs::File::open(file_path)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let mut body = Vec::with_capacity(file_size as usize);
    file.read_to_end(&mut body)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let _ = tokio::fs::remove_file(file_path).await;
    jobs.cleanup(&job_id).await;

    Ok(HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "video/mp4"))
        .append_header((
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ))
        .append_header((header::CONTENT_LENGTH, body.len().to_string()))
        .body(body))
}
