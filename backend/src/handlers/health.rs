use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "video-downloader",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
