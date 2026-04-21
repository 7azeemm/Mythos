use axum::Json;
use serde::Serialize;
use crate::api::endpoints::products::PRODUCTS_PER_PAGE;
use crate::api::error::{ApiError, ApiResult};
use crate::parser::specs_cache::SPECS_CACHE;
use crate::utils::database::get_db_pool;

#[derive(Debug, Serialize)]
pub struct GeneralInfoResponse {
    total_products: i64,
    total_pages: i32,
    filter_options: FilterOptionsData,
}

#[derive(Debug, Serialize)]
struct FilterOptionsData {
    price_range: PriceRange,
    cpus: Vec<FilterOption>,
    gpus: Vec<FilterOption>,
    ram_types: Vec<FilterOption>,
    storage_types: Vec<FilterOption>,
}

#[derive(Debug, Serialize)]
struct FilterOption {
    name: String,
    count: i32,
}

#[derive(Debug, Serialize)]
struct PriceRange {
    min: i32,
    max: i32,
}

pub async fn info() -> ApiResult<Json<GeneralInfoResponse>> {
    let cache = SPECS_CACHE.read().await;

    let total_products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to count products: {}", e)))?;

    let total_pages = ((total_products.0 as f32) / (PRODUCTS_PER_PAGE as f32)).ceil() as i32;

    let price_range: (Option<i32>, Option<i32>) = sqlx::query_as("SELECT MIN(price), MAX(price) FROM products")
        .fetch_one(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch price range: {}", e)))?;

    let mut cpu_options: Vec<FilterOption> = cache
        .cpus
        .iter()
        .map(|(name, ids)| FilterOption {
            name: name.clone(),
            count: ids.len() as i32,
        })
        .collect();
    cpu_options.sort_by(|a, b| b.count.cmp(&a.count));

    let mut gpu_options: Vec<FilterOption> = cache
        .gpus
        .iter()
        .map(|(name, ids)| FilterOption {
            name: name.clone(),
            count: ids.len() as i32,
        })
        .collect();
    gpu_options.sort_by(|a, b| b.count.cmp(&a.count));

    let mut ram_type_options: Vec<FilterOption> = cache
        .ram_types
        .iter()
        .map(|(name, ids)| FilterOption {
            name: name.clone(),
            count: ids.len() as i32,
        })
        .collect();
    ram_type_options.sort_by(|a, b| b.count.cmp(&a.count));

    let mut storage_type_options: Vec<FilterOption> = cache
        .storage_types
        .iter()
        .map(|(name, ids)| FilterOption {
            name: name.clone(),
            count: ids.len() as i32,
        })
        .collect();
    storage_type_options.sort_by(|a, b| b.count.cmp(&a.count));

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
            cpus: cpu_options,
            gpus: gpu_options,
            ram_types: ram_type_options,
            storage_types: storage_type_options,
        },
    }))
}