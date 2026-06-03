use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::error::{AppError, AppResult};

const COOKIES_PATH: &str = "~/.config/vidclaw/cookies.txt";

#[derive(Serialize)]
struct CookiesResponse {
    message: String,
}

pub async fn upload_cookies(body: web::Bytes) -> AppResult<HttpResponse> {
    if body.is_empty() {
        return Err(AppError::InvalidUrl("Cookie file is empty".to_string()));
    }

    // Validate it looks like a Netscape cookies file
    let content = std::str::from_utf8(&body)
        .map_err(|_| AppError::InvalidUrl("Cookie file must be valid UTF-8 text".to_string()))?;

    if !content.contains('\t') {
        return Err(AppError::InvalidUrl(
            "Invalid cookies file. Export a Netscape format cookies.txt using the \
             'Get cookies.txt LOCALLY' browser extension."
                .to_string(),
        ));
    }

    let path = expand_home(COOKIES_PATH);
    let dir = std::path::Path::new(&path).parent().unwrap();

    tokio::fs::create_dir_all(dir)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    tokio::fs::write(&path, &body)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    log::info!("Cookies file saved to {}", path);

    Ok(HttpResponse::Ok().json(CookiesResponse {
        message: "Cookies saved. Instagram and Facebook downloads are now enabled.".to_string(),
    }))
}

pub async fn delete_cookies() -> AppResult<HttpResponse> {
    let path = expand_home(COOKIES_PATH);

    match tokio::fs::remove_file(&path).await {
        Ok(_) => Ok(HttpResponse::Ok().json(CookiesResponse {
            message: "Cookies removed.".to_string(),
        })),
        Err(_) => Ok(HttpResponse::Ok().json(CookiesResponse {
            message: "No cookies file found.".to_string(),
        })),
    }
}

pub async fn cookies_status() -> HttpResponse {
    let path = expand_home(COOKIES_PATH);
    let exists = tokio::fs::metadata(&path).await.is_ok();
    HttpResponse::Ok().json(serde_json::json!({ "active": exists }))
}

fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}{}", home, &path[1..]);
        }
    }
    path.to_string()
}
