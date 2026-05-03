use std::collections::HashMap;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::api::endpoints::products::PRODUCTS_PER_PAGE;
use crate::api::error::{ApiError, ApiResult};
use crate::web_scraper::specs::cache::SPECS_CACHE;
use crate::utils::database::get_db_pool;

#[derive(Deserialize)]
pub struct InfoRequest {
    section: String,
}

#[derive(Debug, Serialize)]
pub struct GeneralInfoResponse {
    total_products: i64,
    total_pages: i32,
    filter_options: FilterOptionsData,
}

#[derive(Debug, Serialize)]
struct FilterOptionsData {
    price_range: PriceRange,
    options: HashMap<String, Vec<FilterOption>>,
}

#[derive(Debug, Serialize)]
pub struct FilterOption {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Serialize)]
struct PriceRange {
    min: i32,
    max: i32,
}

pub async fn info(Query(req): Query<InfoRequest>) -> ApiResult<Json<GeneralInfoResponse>> {
    let total_products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE section = $1")
        .bind(&req.section)
        .fetch_one(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to count products: {}", e)))?;

    let total_pages = ((total_products.0 as f32) / (PRODUCTS_PER_PAGE as f32)).ceil() as i32;

    let price_range: (Option<i32>, Option<i32>) = sqlx::query_as("SELECT MIN(price), MAX(price) FROM products WHERE section = $1")
        .bind(&req.section)
        .fetch_one(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch price range: {}", e)))?;

    let cache = SPECS_CACHE.read().await;
    let options = cache.get_filter_options(&req.section);

    let price_min = price_range.0.unwrap_or(0);
    let price_max = price_range.1.unwrap_or(0);

    Ok(Json(GeneralInfoResponse {
        total_products: total_products.0,
        total_pages,
        filter_options: FilterOptionsData {
            price_range: PriceRange {
                min: price_min,
                max: price_max,
            },
            options
        },
    }))
}