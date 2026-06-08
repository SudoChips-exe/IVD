use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::api::ytdlp;
use crate::error::AppResult;
use crate::util;

#[derive(Deserialize)]
pub struct PlaylistQuery {
    pub url: String,
}

pub async fn playlist_info(query: web::Query<PlaylistQuery>) -> AppResult<HttpResponse> {
    let url = query.url.trim().to_string();
    util::validate_url(&url)?;
    let info = ytdlp::get_playlist_info(&url).await?;
    Ok(HttpResponse::Ok().json(info))
}
