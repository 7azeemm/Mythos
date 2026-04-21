use axum::{extract::{Path, Query}, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use crate::api::error::{ApiError, ApiResult};
use crate::parser::parser::parse_specs;
use crate::utils::database::get_db_pool;
use crate::web_scraper::product::ProductSpecs;

const PRODUCTS_PER_PAGE: i32 = 24;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductMinimalResponse {
    pub id: String,
    pub p_ref: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub status: String,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductDetailedResponse {
    pub id: String,
    pub p_ref: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub status: String,
    pub price: i32,
    pub history: Value,
    #[sqlx(skip)]
    pub specs: Option<ProductSpecs>,
    pub added_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    sort: Option<String>,
}

impl ListQuery {
    fn validate(&self) -> ApiResult<()> {
        if let Some(page) = self.page {
            if page < 1 {
                return Err(ApiError::InvalidQuery("page must be >= 1".to_string()));
            }
        }
        if let Some(sort) = &self.sort {
            if sort != "asc" && sort != "desc" {
                return Err(ApiError::InvalidQuery(
                    "sort must be 'asc' or 'desc'".to_string(),
                ));
            }
        }
        Ok(())
    }
}

pub async fn list(Query(query): Query<ListQuery>) -> ApiResult<Json<Vec<ProductMinimalResponse>>> {
    query.validate()?;
    let page = query.page.unwrap_or(1);
    // let sort = query.sort.unwrap_or("ASC".to_string());

    let offset = (page - 1) * PRODUCTS_PER_PAGE;

    let products: Vec<ProductMinimalResponse> = sqlx::query_as(
        r#"
        SELECT * FROM products
        ORDER BY price ASC
        LIMIT $1 OFFSET $2
        "#
    )
        .bind(PRODUCTS_PER_PAGE)
        .bind(offset)
        .fetch_all(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch products: {}", e)))?;

    Ok(Json(products))
}

pub async fn get_by_id(Path(id): Path<String>) -> ApiResult<Json<ProductDetailedResponse>> {
    let mut product: ProductDetailedResponse = sqlx::query_as(
        "SELECT * FROM products WHERE id = $1"
    )
        .bind(&id)
        .fetch_optional(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch product: {}", e)))?
        .ok_or_else(|| ApiError::NotFound(format!("Product '{}' not found", id)))?;

    product.specs = Some(ProductSpecs::PC(
            parse_specs(&product.description)
                .map_err(|e| ApiError::InternalError(format!("Failed to parse product: {}", e)))?
        ));

    Ok(Json(product))
}