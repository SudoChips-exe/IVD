pub mod download;
pub mod health;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/download", web::post().to(download::download_video))
            .route("/health", web::get().to(health::health_check)),
    );
}
