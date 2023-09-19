use actix_web::{get, HttpResponse, web};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::entities::plugin::Plugin;
use actix_web::error::HttpError;
#[derive(Serialize, Deserialize)]
pub struct PluginMetadata {
    pub total_count: i32,
    pub total_downloads: i32,
    pub page_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct PluginDto {
    pub name: String,
    pub description: String,
    pub version: String,
    pub time: NaiveDateTime,
    pub author: String,
    pub author_email: String,
    pub official: bool,
    pub popularity_score: f32,
    pub keywords: Vec<String>,
    pub image: Option<String>,
    pub readme: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct PluginResponse {
    pub metadata: PluginMetadata,
    pub plugins: Vec<PluginDto>
}

#[derive(Serialize, Deserialize)]
pub enum SortOrder {
    DownloadsASC,
    DownloadsDESC,
    CreatedASC,
    CreatedDESC,
    UpdatedASC,
    UpdatedDESC,
}


#[derive(Deserialize)]
pub struct AvailableQuery{
    pub last_plugin_name: Option<String>,
    pub page_size: Option<i32>,
    pub order: Option<SortOrder>,
    pub query: Option<String>,
    pub official: Option<bool>,
}

#[get("/plugins")]
pub async fn get_available_plugins(query: web::Query<AvailableQuery>) -> Result<HttpResponse,
    HttpError>{
    let plugins = Plugin::get_available_plugins(query).await;

    return Ok(HttpResponse::Ok().json(plugins))
}