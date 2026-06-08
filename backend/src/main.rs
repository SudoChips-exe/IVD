mod api;
mod cache;
mod config;
mod error;
mod handlers;
mod jobs;
mod middleware;
mod models;
mod util;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;

fn init_cookies_from_env() {
    let Ok(b64) = std::env::var("COOKIES_B64") else { return };
    let b64 = b64.trim();
    if b64.is_empty() { return }
    use std::io::Write;
    let decoded = match base64_decode(b64) {
        Some(d) => d,
        None => { log::warn!("COOKIES_B64 is set but failed to decode"); return }
    };
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let dir = format!("{}/.config/vidclaw", home);
    let path = format!("{}/cookies.txt", dir);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::warn!("Could not create cookies dir: {}", e); return;
    }
    match std::fs::File::create(&path).and_then(|mut f| f.write_all(&decoded)) {
        Ok(_) => log::info!("cookies.txt loaded from COOKIES_B64"),
        Err(e) => log::warn!("Could not write cookies.txt: {}", e),
    }
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let s: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u32;
    for &b in bytes {
        let val = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return None,
        } as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 { bits -= 8; out.push((buf >> bits) as u8); buf &= (1 << bits) - 1; }
    }
    Some(out)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv::dotenv().ok();
    init_cookies_from_env();
    let config = config::Config::from_env();

    log::info!("Starting VIDCLAW Server");
    log::info!("Listening on {}:{}", config.server_host, config.server_port);

    let addr = format!("{}:{}", config.server_host, config.server_port);

    let app_state = web::Data::new(config.clone());
    let metadata_cache = web::Data::new(cache::MetadataCache::new(config.cache_ttl_seconds));
    let job_store = web::Data::new(jobs::JobStore::new());
    let rate_limiter = middleware::RateLimiter::new(
        config.max_requests_per_minute,
        config.max_requests_per_ip_per_minute,
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(metadata_cache.clone())
            .app_data(job_store.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::ORIGIN,
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .wrap(rate_limiter.clone())
            .configure(handlers::configure_routes)
    })
    .bind(&addr)?
    .run()
    .await
}
