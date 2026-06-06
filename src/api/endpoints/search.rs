// use axum::{extract::Query, Json};
// use serde::Deserialize;
// use crate::api::error::{ApiError, ApiResult};
// use crate::api::endpoints::products::{ProductMinimalResponse, PRODUCTS_PER_PAGE};
// use crate::utils::database::get_db_pool;
// 
// #[derive(Deserialize)]
// pub struct SearchQuery {
//     q: String,
//     page: Option<i32>,
// }
// 
// impl SearchQuery {
//     fn validate(&self) -> ApiResult<()> {
//         if self.q.trim().is_empty() {
//             return Err(ApiError::InvalidQuery(
//                 "search query 'q' cannot be empty".to_string(),
//             ));
//         }
//         if self.q.len() > 256 {
//             return Err(ApiError::InvalidQuery(
//                 "search query 'q' must be less than 256 characters".to_string(),
//             ));
//         }
//         if let Some(page) = self.page {
//             if page < 1 {
//                 return Err(ApiError::InvalidQuery("page must be >= 1".to_string()));
//             }
//         }
//         Ok(())
//     }
// }
// 
// pub async fn search(Query(query): Query<SearchQuery>) -> ApiResult<Json<Vec<ProductMinimalResponse>>> {
//     query.validate()?;
// 
//     let page = query.page.unwrap_or(1);
//     let offset = (page - 1) * PRODUCTS_PER_PAGE;
//     let search_term = format!("%{}%", query.q);
// 
//     let products: Vec<ProductMinimalResponse> = sqlx::query_as(
//         r#"
//         SELECT id, p_ref, section, source, title, description, url, image, status, price
//         FROM products
//         WHERE title ILIKE $1 OR description ILIKE $1
//         ORDER BY
//             CASE
//                 WHEN title ILIKE $1 THEN 0
//                 ELSE 1
//             END,
//             price ASC
//         LIMIT $2 OFFSET $3
//         "#,
//     )
//         .bind(&search_term)
//         .bind(PRODUCTS_PER_PAGE)
//         .bind(offset)
//         .fetch_all(get_db_pool())
//         .await
//         .map_err(|e| ApiError::DatabaseError(format!("Failed to search products: {}", e)))?;
// 
//     Ok(Json(products))
// }