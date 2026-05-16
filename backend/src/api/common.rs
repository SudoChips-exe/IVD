use crate::error::{AppError, AppResult};
use regex::Regex;
use reqwest::Client;
use serde_json::Value;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";

pub fn default_user_agent() -> &'static str {
    USER_AGENT
}

pub fn build_client() -> AppResult<Client> {
    Client::builder()
        .user_agent(default_user_agent())
        .build()
        .map_err(|e| AppError::InternalServerError(format!("Failed to build HTTP client: {}", e)))
}

pub async fn fetch_html(url: &str) -> AppResult<String> {
    let client = build_client()?;
    let response = client
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await
        .map_err(|e| AppError::NetworkError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(AppError::PlatformError(format!(
            "HTTP {} response from {}",
            response.status(),
            url
        )));
    }

    response
        .text()
        .await
        .map_err(|e| AppError::NetworkError(e.to_string()))
}

pub fn extract_meta_content(html: &str, property: &str) -> Option<String> {
    let re = Regex::new(&format!(
        r#"(?si)<meta[^>]*(?:property|name)=["']{}["'][^>]*content=["']([^"']+)["'][^>]*>"#,
        regex::escape(property)
    ))
    .unwrap();

    re.captures(html)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

pub fn extract_script_json(html: &str, id: &str) -> Option<String> {
    let re = Regex::new(&format!(
        r#"(?si)<script[^>]*id=["']{}["'][^>]*>(.*?)</script>"#,
        regex::escape(id)
    ))
    .unwrap();

    re.captures(html).and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
}

pub fn extract_ld_json_scripts(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?si)<script[^>]*type=["']application/ld\+json["'][^>]*>(.*?)</script>"#).unwrap();
    re.captures_iter(html)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
        .collect()
}

pub fn parse_json_value(text: &str) -> Option<Value> {
    serde_json::from_str::<Value>(text).ok()
}
