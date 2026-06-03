use actix_web::{web, HttpResponse};

use crate::jobs::JobStore;

pub async fn cancel_download(
    job_id: web::Path<String>,
    jobs: web::Data<JobStore>,
) -> HttpResponse {
    if jobs.cancel(&job_id).await {
        log::info!("Download cancelled: {}", job_id);
        HttpResponse::Ok().json(serde_json::json!({ "cancelled": true }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "cancelled": false, "message": "Job not found" }))
    }
}
