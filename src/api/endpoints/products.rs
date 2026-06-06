// use std::collections::{HashMap, HashSet};
// use axum::{extract::{Path, Query}, Json};
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use sqlx::FromRow;
// use crate::api::error::{ApiError, ApiResult};
// use crate::utils::database::get_db_pool;
//
// pub const PRODUCTS_PER_PAGE: i32 = 24;
//
// #[derive(Debug, Serialize, Deserialize, FromRow)]
// pub struct ProductMinimalResponse {
//     pub id: String,
//     pub p_ref: String,
//     pub section: String,
//     pub source: String,
//     pub title: String,
//     pub description: String,
//     pub url: String,
//     pub image: String,
//     pub status: String,
//     pub price: i32,
// }
//
// #[derive(Debug, Serialize, Deserialize, FromRow)]
// pub struct ProductDetailedResponse {
//     id: String,
//     p_ref: String,
//     section: String,
//     source: String,
//     title: String,
//     description: String,
//     url: String,
//     image: String,
//     status: String,
//     price: i32,
//     history: Value,
//     #[sqlx(skip)]
//     specs: Option<ProductSpecs>,
//     added_at: Option<DateTime<Utc>>,
//     updated_at: Option<DateTime<Utc>>,
// }
//
// #[derive(Deserialize)]
// pub struct ListQuery {
//     section: String,
//     page: Option<i32>,
//     sort: Option<String>,
//     min_price: Option<i32>,
//     max_price: Option<i32>,
//     options: Option<HashMap<String, Vec<String>>>,
// }
//
// impl ListQuery {
//     fn validate(&self) -> ApiResult<()> {
//         if let Some(page) = self.page {
//             if page < 1 {
//                 return Err(ApiError::InvalidQuery("page must be >= 1".to_string()));
//             }
//         }
//         if let Some(sort) = &self.sort {
//             if sort != "price_asc" && sort != "price_desc" {
//                 return Err(ApiError::InvalidQuery(
//                     "sort must be 'price_asc' or 'price_desc'".to_string(),
//                 ));
//             }
//         }
//         if let Some(min) = self.min_price {
//             if min < 0 {
//                 return Err(ApiError::InvalidQuery(
//                     "min_price must be >= 0".to_string(),
//                 ));
//             }
//         }
//         if let Some(max) = self.max_price {
//             if max < 0 {
//                 return Err(ApiError::InvalidQuery(
//                     "max_price must be >= 0".to_string(),
//                 ));
//             }
//         }
//         Ok(())
//     }
// }
//
// pub async fn list(Query(query): Query<ListQuery>) -> ApiResult<Json<Vec<ProductMinimalResponse>>> {
//     query.validate()?;
//
//     let page = query.page.unwrap_or(1);
//     let sort = query.sort.unwrap_or_else(|| "price_asc".to_string());
//     let offset = (page - 1) * PRODUCTS_PER_PAGE;
//
//     let mut matching_ids: Option<HashSet<String>> = None;
//     if let Some(options) = query.options {
//         let cache = SPECS_CACHE.read().await;
//
//         // Iterate over each filter category provided in query.options
//         for (filter_type, filter_values) in options {
//             let mut current_filter_set: HashSet<String> = HashSet::new();
//
//             // 1. Gather all products matching ANY of the values for this specific filter (OR logic)
//             for value in filter_values {
//                 let products = cache.filter_products(&query.section, &filter_type, &value);
//                 current_filter_set.extend(products);
//             }
//
//             // 2. Intersect this category's results with the overall matching_ids (AND logic across categories)
//             matching_ids = Some(match matching_ids {
//                 None => current_filter_set,
//                 Some(existing) => existing.intersection(&current_filter_set).cloned().collect(),
//             });
//
//             // 3. If at any point the intersection is empty, stop checking further filters
//             if let Some(ref ids) = matching_ids {
//                 if ids.is_empty() {
//                     break;
//                 }
//             }
//         }
//     }
//
//     let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
//         r#"
//         SELECT id, p_ref, section, source, title, description, url, image, status, price
//         FROM products
//         WHERE 1=1
//         "#
//     );
//
//     query_builder.push(" AND section = ");
//     query_builder.push_bind(query.section);
//
//     if let Some(min_price) = query.min_price {
//         query_builder.push(" AND price >= ");
//         query_builder.push_bind(min_price);
//     }
//
//     if let Some(max_price) = query.max_price {
//         query_builder.push(" AND price <= ");
//         query_builder.push_bind(max_price);
//     }
//
//     if let Some(ids) = &matching_ids {
//         if ids.is_empty() {
//             return Ok(Json(vec![]));
//         }
//
//         query_builder.push(" AND id IN (");
//         let mut separated = query_builder.separated(", ");
//         for id in ids {
//             separated.push_bind(id);
//         }
//         separated.push_unseparated(")");
//     }
//
//     let sort_clause = match sort.as_str() {
//         "price_desc" => " ORDER BY price DESC",
//         _ => " ORDER BY price ASC",
//     };
//
//     query_builder.push(sort_clause);
//     query_builder.push(" LIMIT ");
//     query_builder.push_bind(PRODUCTS_PER_PAGE as i64);
//     query_builder.push(" OFFSET ");
//     query_builder.push_bind(offset as i64);
//
//     let products: Vec<ProductMinimalResponse> = query_builder
//         .build_query_as()
//         .fetch_all(get_db_pool())
//         .await
//         .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch products: {}", e)))?;
//
//     Ok(Json(products))
// }
//
// pub async fn get_by_id(Path(id): Path<String>) -> ApiResult<Json<ProductDetailedResponse>> {
//     let mut product: ProductDetailedResponse = sqlx::query_as(
//         "SELECT * FROM products WHERE id = $1"
//     )
//         .bind(&id)
//         .fetch_optional(get_db_pool())
//         .await
//         .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch product: {}", e)))?
//         .ok_or_else(|| ApiError::NotFound(format!("Product '{}' not found", id)))?;
//
//     if let Some(specs) = SPECS_CACHE.read().await.specs.get(&id).cloned() {
//         product.specs = Some(specs)
//     }
//
//     Ok(Json(product))
// }