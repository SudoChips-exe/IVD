use actix_web::{test, web, App};
use serde_json::json;
use video_downloader::{cache::MetadataCache, config::Config, handlers, jobs::JobStore};

fn test_config() -> Config {
    Config {
        server_host: "127.0.0.1".to_string(),
        server_port: 8080,
        frontend_url: "http://localhost:5173".to_string(),
        instagram_api_key: None,
        instagram_business_account_id: None,
        tiktok_api_key: None,
        youtube_api_key: None,
        twitter_api_key: None,
        twitter_api_secret: None,
        twitter_bearer_token: None,
        facebook_api_key: None,
        facebook_app_id: None,
        max_requests_per_minute: 10000,
        max_requests_per_ip_per_minute: 10000,
        cache_ttl_seconds: 900,
        platform_status_cache_ttl: 3600,
    }
}

macro_rules! app {
    () => {
        test::init_service(
            App::new()
                .app_data(web::Data::new(test_config()))
                .app_data(web::Data::new(MetadataCache::new(900)))
                .app_data(web::Data::new(JobStore::new()))
                .configure(handlers::configure_routes),
        )
        .await
    };
}

// ── Health ────────────────────────────────────────────────────────────────────

#[actix_web::test]
async fn health_returns_200() {
    let app = app!();
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn health_body_has_status_and_service_fields() {
    let app = app!();
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "video-downloader");
    assert!(body["timestamp"].is_string());
}

// ── Download — validation errors (synchronous, no external calls) ─────────────

#[actix_web::test]
async fn download_empty_url_returns_400() {
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/download")
        .set_json(json!({ "url": "" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn download_whitespace_url_returns_400() {
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/download")
        .set_json(json!({ "url": "   " }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn download_no_protocol_returns_400() {
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/download")
        .set_json(json!({ "url": "instagram.com/reel/abc123" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn download_valid_url_returns_job_id() {
    // Valid URLs start a background job immediately and return 200 + job_id.
    // Errors (unsupported site, auth) come later via SSE.
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/download")
        .set_json(json!({ "url": "https://example.com/video/123" }))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(body.get("job_id").is_some(), "expected job_id in response");
    assert!(body["job_id"].is_string());
}

#[actix_web::test]
async fn download_error_body_contains_error_and_message_fields() {
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/download")
        .set_json(json!({ "url": "" }))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(body.get("error").is_some(), "missing 'error' field");
    assert!(body.get("message").is_some(), "missing 'message' field");
}

// ── Progress / File endpoints ─────────────────────────────────────────────────

#[actix_web::test]
async fn progress_unknown_job_returns_404() {
    let app = app!();
    let req = test::TestRequest::get()
        .uri("/api/progress/nonexistent-job-id")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn file_unknown_job_returns_404() {
    let app = app!();
    let req = test::TestRequest::get()
        .uri("/api/file/nonexistent-job-id")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

// ── Cookies ───────────────────────────────────────────────────────────────────

#[actix_web::test]
async fn cookies_status_returns_json() {
    let app = app!();
    let req = test::TestRequest::get().uri("/api/cookies/status").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(body.get("active").is_some());
}

#[actix_web::test]
async fn cookies_upload_empty_body_returns_400() {
    let app = app!();
    let req = test::TestRequest::post()
        .uri("/api/cookies")
        .insert_header(("content-type", "text/plain"))
        .set_payload("")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

// ── Routing ───────────────────────────────────────────────────────────────────

#[actix_web::test]
async fn unknown_route_returns_404() {
    let app = app!();
    let req = test::TestRequest::get().uri("/api/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn download_via_get_returns_4xx() {
    let app = app!();
    let req = test::TestRequest::get().uri("/api/download").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}
