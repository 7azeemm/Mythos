// use axum::extract::Query;
// use axum::Json;
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use sqlx::FromRow;
// use crate::api::error::{ApiError, ApiResult};
// use crate::utils::database::get_db_pool;
// 
// const RECENT_LIMIT: i32 = 24;
// 
// #[derive(Debug, Serialize, Deserialize, FromRow)]
// pub struct RecentProductResponse {
//     pub id: String,
//     pub p_ref: String,
//     pub section: String,
//     pub source: String,
//     pub title: String,
//     pub description: String,
//     pub image: String,
//     pub status: String,
//     pub price: i32,
//     pub history: Value,
//     #[sqlx(default)]
//     pub added_at: Option<DateTime<Utc>>,
//     #[sqlx(default)]
//     pub removed_at: Option<DateTime<Utc>>,
// }
// 
// #[derive(Deserialize)]
// pub struct RecentQuery {
//     section: String,
//     #[serde(rename = "type")]
//     query_type: Option<String>,
// }
// 
// pub async fn recent(Query(query): Query<RecentQuery>) -> ApiResult<Json<Vec<RecentProductResponse>>> {
//     let query_type = query.query_type.as_deref().unwrap_or("both");
// 
//     match query_type {
//         "added" => fetch_recent_products(query.section, "products", "added_at").await,
//         "removed" => fetch_recent_products(query.section, "products_archive", "removed_at").await,
//         "both" => {
//             let added_future = fetch_recent_products(query.section.clone(), "products", "added_at");
//             let removed_future = fetch_recent_products(query.section, "products_archive", "removed_at");
// 
//             let (added, removed) = tokio::join!(added_future, removed_future);
// 
//             let mut added_products = added?;
//             let mut removed_products = removed?;
// 
//             let mut all_products = {
//                 added_products.append(&mut removed_products);
//                 added_products
//             };
// 
//             all_products.sort_by(|a, b| {
//                 let a_date = a.added_at.unwrap_or(DateTime::<Utc>::MIN_UTC);
//                 let b_date = b.added_at.unwrap_or(DateTime::<Utc>::MIN_UTC);
//                 b_date.cmp(&a_date)
//             });
// 
//             all_products.truncate(RECENT_LIMIT as usize);
// 
//             Ok(all_products)
//         }
//         _ => Err(ApiError::InvalidQuery(
//             "type must be 'added', 'removed', or 'both' (or omit for 'both')".to_string(),
//         ))
//     }
// }
// 
// async fn fetch_recent_products(section: String, table: &str, date_column: &str) -> ApiResult<Json<Vec<RecentProductResponse>>> {
//     let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
//         "SELECT id, p_ref, section, source, title, description, image, status, price, history, "
//     );
// 
//     query_builder.push(date_column);
//     query_builder.push(" as added_at FROM ");
//     query_builder.push(table);
//     query_builder.push(" WHERE ");
//     query_builder.push("section = ");
//     query_builder.push_bind(section);
//     query_builder.push(" AND ");
//     query_builder.push(date_column);
//     query_builder.push(" IS NOT NULL ORDER BY ");
//     query_builder.push(date_column);
//     query_builder.push(" DESC LIMIT ");
// 
//     query_builder.push_bind(RECENT_LIMIT as i64);
// 
//     let products: Vec<RecentProductResponse> = query_builder
//         .build_query_as()
//         .fetch_all(get_db_pool())
//         .await
//         .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch recent products from {}: {}", table, e)))?;
// 
//     Ok(Json(products))
// }