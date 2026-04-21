use std::collections::HashSet;
use axum::{extract::{Path, Query}, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use crate::api::error::{ApiError, ApiResult};
use crate::parser::parser::parse_specs;
use crate::parser::specs_cache::SPECS_CACHE;
use crate::utils::database::get_db_pool;
use crate::web_scraper::product::ProductSpecs;

pub const PRODUCTS_PER_PAGE: i32 = 24;

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
    id: String,
    p_ref: String,
    title: String,
    description: String,
    url: String,
    image: String,
    status: String,
    price: i32,
    history: Value,
    #[sqlx(skip)]
    specs: Option<ProductSpecs>,
    added_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    sort: Option<String>,
    min_price: Option<i32>,
    max_price: Option<i32>,
    cpu: Option<String>,
    gpu: Option<String>,
    ram_type: Option<String>,
    storage_type: Option<String>,
}

impl ListQuery {
    fn validate(&self) -> ApiResult<()> {
        if let Some(page) = self.page {
            if page < 1 {
                return Err(ApiError::InvalidQuery("page must be >= 1".to_string()));
            }
        }
        if let Some(sort) = &self.sort {
            if sort != "price_asc" && sort != "price_desc" {
                return Err(ApiError::InvalidQuery(
                    "sort must be 'price_asc' or 'price_desc'".to_string(),
                ));
            }
        }
        if let Some(min) = self.min_price {
            if min < 0 {
                return Err(ApiError::InvalidQuery(
                    "min_price must be >= 0".to_string(),
                ));
            }
        }
        if let Some(max) = self.max_price {
            if max < 0 {
                return Err(ApiError::InvalidQuery(
                    "max_price must be >= 0".to_string(),
                ));
            }
        }
        Ok(())
    }
}

pub async fn list(Query(query): Query<ListQuery>) -> ApiResult<Json<Vec<ProductMinimalResponse>>> {
    query.validate()?;

    let page = query.page.unwrap_or(1);
    let sort = query.sort.unwrap_or_else(|| "price_asc".to_string());
    let offset = (page - 1) * PRODUCTS_PER_PAGE;

    let mut matching_ids: Option<HashSet<String>> = None;
    {
        let cache = SPECS_CACHE.read().await;

        if let Some(cpu) = &query.cpu {
            let cpu_products = cache.filter_products_by_cpu(cpu);
            let cpu_set: HashSet<String> = cpu_products.into_iter().collect();
            matching_ids = Some(match matching_ids {
                None => cpu_set,
                Some(existing) => existing.intersection(&cpu_set).cloned().collect(),
            });
        }

        if let Some(gpu) = &query.gpu {
            let gpu_products = cache.filter_products_by_gpu(gpu);
            let gpu_set: HashSet<String> = gpu_products.into_iter().collect();
            matching_ids = Some(match matching_ids {
                None => gpu_set,
                Some(existing) => existing.intersection(&gpu_set).cloned().collect(),
            });
        }

        if let Some(ram_type) = &query.ram_type {
            let ram_products = cache.filter_products_by_ram_type(ram_type);
            let ram_set: HashSet<String> = ram_products.into_iter().collect();
            matching_ids = Some(match matching_ids {
                None => ram_set,
                Some(existing) => existing.intersection(&ram_set).cloned().collect(),
            });
        }

        if let Some(storage_type) = &query.storage_type {
            let storage_products = cache.filter_products_by_storage_type(storage_type);
            let storage_set: HashSet<String> = storage_products.into_iter().collect();
            matching_ids = Some(match matching_ids {
                None => storage_set,
                Some(existing) => existing.intersection(&storage_set).cloned().collect(),
            });
        }
    }

    let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
        r#"
        SELECT id, p_ref, title, description, url, image, status, price
        FROM products
        WHERE 1=1
        "#
    );

    if let Some(min_price) = query.min_price {
        query_builder.push(" AND price >= ");
        query_builder.push_bind(min_price);
    }

    if let Some(max_price) = query.max_price {
        query_builder.push(" AND price <= ");
        query_builder.push_bind(max_price);
    }

    if let Some(ids) = &matching_ids {
        if ids.is_empty() {
            return Ok(Json(vec![]));
        }

        query_builder.push(" AND id IN (");
        let mut separated = query_builder.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");
    }

    let sort_clause = match sort.as_str() {
        "price_desc" => " ORDER BY price DESC",
        _ => " ORDER BY price ASC",
    };

    query_builder.push(sort_clause);
    query_builder.push(" LIMIT ");
    query_builder.push_bind(PRODUCTS_PER_PAGE as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset as i64);

    let products: Vec<ProductMinimalResponse> = query_builder
        .build_query_as()
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

    match parse_specs(&product.description) {
        Err(err) => eprintln!("Failed to parse product {id}: {err}"),
        Ok(specs) => product.specs = Some(ProductSpecs::PC(specs))
    };

    Ok(Json(product))
}