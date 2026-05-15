mod api;
mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod util;

use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use std::sync::Mutex;

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
    let app_state = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(middleware::RateLimiter::new())
            .configure(handlers::configure_routes)
    })
    .bind(&addr)?
    .run()
    .await
}
