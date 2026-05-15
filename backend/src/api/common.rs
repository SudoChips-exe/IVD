use crate::error::{AppError, AppResult};
use reqwest::Client;
use serde_json::json;

pub struct HttpClient;

impl HttpClient {
    pub fn new() -> Client {
        Client::new()
    }

    pub async fn get_json(client: &Client, url: &str) -> AppResult<serde_json::Value> {
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::PlatformError(format!(
                "HTTP {} response",
                response.status()
            )));
        }

        response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }

    pub async fn get_bytes(client: &Client, url: &str) -> AppResult<bytes::Bytes> {
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::PlatformError(format!(
                "HTTP {} response",
                response.status()
            )));
        }

        response
            .bytes()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))
    }
}

/// Parse video URL to extract content
pub fn parse_instagram_response(data: &serde_json::Value) -> AppResult<(String, String, String)> {
    // Extract video_url, title, author from response
    // This is a placeholder - will be implemented based on actual API response format
    let video_url = data["video_url"]
        .as_str()
        .ok_or_else(|| AppError::PlatformError("Missing video_url".to_string()))?
        .to_string();

    let title = data["title"]
        .as_str()
        .unwrap_or("Instagram Video")
        .to_string();

    let author = data["author"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string();

    Ok((video_url, title, author))
}
