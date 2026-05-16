use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    // User errors (4xx)
    InvalidUrl(String),
    PlatformNotSupported(String),
    VideoNotFound(String),
    PrivateVideo(String),
    InvalidVideoFormat(String),
    
    // Rate limit
    RateLimited { retry_after: u64 },
    
    // Platform errors
    PlatformError(String),
    ApiAuthenticationFailed(String),
    ApiQuotaExceeded(String),
    PlatformUnavailable(String),
    
    // Server errors (5xx)
    InternalServerError(String),
    StreamingError(String),
    NetworkError(String),
    ConfigurationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            AppError::PlatformNotSupported(platform) => write!(f, "Platform not supported: {}", platform),
            AppError::VideoNotFound(msg) => write!(f, "Video not found: {}", msg),
            AppError::PrivateVideo(msg) => write!(f, "Private or restricted video: {}", msg),
            AppError::InvalidVideoFormat(msg) => write!(f, "Invalid video format: {}", msg),
            AppError::RateLimited { .. } => write!(f, "Rate limit exceeded"),
            AppError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
            AppError::ApiAuthenticationFailed(msg) => write!(f, "API authentication failed: {}", msg),
            AppError::ApiQuotaExceeded(msg) => write!(f, "API quota exceeded: {}", msg),
            AppError::PlatformUnavailable(msg) => write!(f, "Platform unavailable: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::StreamingError(msg) => write!(f, "Streaming error: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidUrl(_) => StatusCode::BAD_REQUEST,
            AppError::PlatformNotSupported(_) => StatusCode::BAD_REQUEST,
            AppError::VideoNotFound(_) => StatusCode::NOT_FOUND,
            AppError::PrivateVideo(_) => StatusCode::FORBIDDEN,
            AppError::InvalidVideoFormat(_) => StatusCode::BAD_REQUEST,
            AppError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            AppError::PlatformError(_) => StatusCode::BAD_GATEWAY,
            AppError::ApiAuthenticationFailed(_) => StatusCode::UNAUTHORIZED,
            AppError::ApiQuotaExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::PlatformUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::StreamingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
            AppError::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_code = match self {
            AppError::InvalidUrl(_) => "INVALID_URL",
            AppError::PlatformNotSupported(_) => "PLATFORM_NOT_SUPPORTED",
            AppError::VideoNotFound(_) => "VIDEO_NOT_FOUND",
            AppError::PrivateVideo(_) => "PRIVATE_VIDEO",
            AppError::InvalidVideoFormat(_) => "INVALID_VIDEO_FORMAT",
            AppError::RateLimited { .. } => "RATE_LIMITED",
            AppError::PlatformError(_) => "PLATFORM_ERROR",
            AppError::ApiAuthenticationFailed(_) => "API_AUTHENTICATION_FAILED",
            AppError::ApiQuotaExceeded(_) => "API_QUOTA_EXCEEDED",
            AppError::PlatformUnavailable(_) => "PLATFORM_UNAVAILABLE",
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            AppError::StreamingError(_) => "STREAMING_ERROR",
            AppError::NetworkError(_) => "NETWORK_ERROR",
            AppError::ConfigurationError(_) => "CONFIGURATION_ERROR",
        };

        let retry_after = match self {
            AppError::RateLimited { retry_after } => Some(*retry_after),
            _ => None,
        };

        let mut builder = HttpResponse::build(status);

        if let Some(retry_after) = retry_after {
            builder.insert_header(("Retry-After", retry_after.to_string()));
        }

        builder.json(json!({
            "error": error_code,
            "message": self.to_string(),
        }))
    }
}

pub type AppResult<T> = Result<T, AppError>;
