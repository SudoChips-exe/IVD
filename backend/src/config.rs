use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub frontend_url: String,
    
    // API Keys
    pub instagram_api_key: Option<String>,
    pub instagram_business_account_id: Option<String>,
    pub tiktok_api_key: Option<String>,
    pub youtube_api_key: Option<String>,
    pub twitter_api_key: Option<String>,
    pub twitter_api_secret: Option<String>,
    pub twitter_bearer_token: Option<String>,
    pub facebook_api_key: Option<String>,
    pub facebook_app_id: Option<String>,
    
    // Rate Limiting
    pub max_requests_per_minute: u32,
    pub max_requests_per_ip_per_minute: u32,
    
    // Cache
    pub cache_ttl_seconds: u64,
    pub platform_status_cache_ttl: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            
            instagram_api_key: env::var("INSTAGRAM_API_KEY").ok(),
            instagram_business_account_id: env::var("INSTAGRAM_BUSINESS_ACCOUNT_ID").ok(),
            tiktok_api_key: env::var("TIKTOK_API_KEY").ok(),
            youtube_api_key: env::var("YOUTUBE_API_KEY").ok(),
            twitter_api_key: env::var("TWITTER_API_KEY").ok(),
            twitter_api_secret: env::var("TWITTER_API_SECRET").ok(),
            twitter_bearer_token: env::var("TWITTER_BEARER_TOKEN").ok(),
            facebook_api_key: env::var("FACEBOOK_API_KEY").ok(),
            facebook_app_id: env::var("FACEBOOK_APP_ID").ok(),
            
            max_requests_per_minute: env::var("MAX_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            max_requests_per_ip_per_minute: env::var("MAX_REQUESTS_PER_IP_PER_MINUTE")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            
            cache_ttl_seconds: env::var("CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .unwrap_or(900),
            platform_status_cache_ttl: env::var("PLATFORM_STATUS_CACHE_TTL")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
        }
    }
}
