use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::api::ytdlp;
use crate::error::AppResult;
use crate::util;

#[derive(Deserialize)]
pub struct InfoRequest {
    pub url: String,
}

pub async fn video_info(req: web::Json<InfoRequest>) -> AppResult<HttpResponse> {
    let url = req.url.trim().to_string();
    util::validate_url(&url)?;
    let info = ytdlp::get_info(&url).await?;
    Ok(HttpResponse::Ok().json(info))
}
