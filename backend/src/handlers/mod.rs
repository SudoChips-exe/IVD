pub mod cancel;
pub mod cookies;
pub mod download;
pub mod file_delivery;
pub mod health;
pub mod progress;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/download", web::post().to(download::download_video))
            .route("/cancel/{job_id}", web::post().to(cancel::cancel_download))
            .route("/progress/{job_id}", web::get().to(progress::progress_sse))
            .route("/file/{job_id}", web::get().to(file_delivery::download_file))
            .route("/cookies", web::post().to(cookies::upload_cookies))
            .route("/cookies", web::delete().to(cookies::delete_cookies))
            .route("/cookies/status", web::get().to(cookies::cookies_status))
            .route("/health", web::get().to(health::health_check)),
    );
}
