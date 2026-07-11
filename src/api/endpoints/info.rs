use axum::Json;
use serde::Serialize;
use strum::IntoEnumIterator;
use crate::api::error::ApiResult;
use crate::web_scraper::sections::Section;

#[derive(Serialize)]
pub struct ServerInfo {
    sections: Vec<String>
}

pub async fn get_info() -> ApiResult<Json<ServerInfo>> {
    Ok(Json(ServerInfo {
        sections: Section::iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }))
}