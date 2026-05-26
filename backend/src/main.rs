mod api;
mod cache;
mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod util;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = config::Config::from_env();

    log::info!("Starting Video Downloader Server");
    log::info!("Listening on {}:{}", config.server_host, config.server_port);

    let addr = format!("{}:{}", config.server_host, config.server_port);

    // Create shared state
    let app_state = web::Data::new(config.clone());
    let metadata_cache = web::Data::new(cache::MetadataCache::new(config.cache_ttl_seconds));
    let rate_limiter = middleware::RateLimiter::new(
        config.max_requests_per_minute,
        config.max_requests_per_ip_per_minute,
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(metadata_cache.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
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
