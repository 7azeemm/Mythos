use crate::api::error::{ApiError, ApiResult};
use crate::utils::database::get_db_pool;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use axum_extra::extract::Query;
use serde_json::Value;
use sqlx::{Postgres, QueryBuilder};
use strum::IntoEnumIterator;

const PAGE_SIZE: i64 = 60;
const OTHERS_LABEL: &str = "Others";

#[derive(Serialize, Debug)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub groups: Vec<(String, Vec<Product>)>,
    pub total: usize,
    pub total_pages: usize,
    pub page: usize,
}

#[derive(Serialize, Debug)]
pub struct SectionResponse {
    pub filters: Vec<FilterOption>,
    pub render_specs: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct FilterOption {
    pub option: String,
    pub values: Vec<String>
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub groups: bool,
    pub search: Option<String>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub site: Vec<String>,
    pub stock: Vec<String>,
    pub sort: Option<String>,
    pub specs: Option<String>,
}

pub async fn get_products(
    Path(section): Path<String>,
    Query(params): Query<PaginationQuery>,
) -> ApiResult<Json<ProductListResponse>> {
    let page = params.page.unwrap_or(1).max(1);

    let order_direction = match params.sort.as_deref() {
        Some("price_desc") => "DESC",
        _ => "ASC",
    };

    // Fetch ALL matching products
    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT * FROM products WHERE 1=1"
    );
    apply_filters(&mut builder, &section, &params);
    builder.push(&format!(" ORDER BY price {}", order_direction));

    let all_products: Vec<Product> = builder
        .build_query_as()
        .fetch_all(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Fetch error: {}", e)))?;

    let total = all_products.len();

    if !params.groups {
        let total_pages = ((total as i64 + PAGE_SIZE - 1) / PAGE_SIZE) as usize;
        let offset = ((page - 1) * PAGE_SIZE) as usize;
        let products = all_products
            .into_iter()
            .skip(offset)
            .take(PAGE_SIZE as usize)
            .collect();

        Ok(Json(ProductListResponse {
            products,
            groups: Vec::new(),
            page: page as usize,
            total,
            total_pages,
        }))
    } else {
        // 1. Group products
        let all_grouped_products = group_products(all_products);

        let mut paginated_groups = Vec::new();
        let mut current_page = 1_usize;
        let mut current_page_size = 0;
        let target_page = page as usize;

        for (group_name, group_products) in all_grouped_products {
            let group_size = group_products.len();

            // 2. Add to response if we are on the requested page target
            if current_page == target_page {
                paginated_groups.push((group_name, group_products));
            }

            // 3. Accumulate items and move to the next page if `PAGE_SIZE` limit is reached
            current_page_size += group_size;
            if current_page_size >= PAGE_SIZE as usize {
                current_page += 1;
                current_page_size = 0;
            }
        }

        // 4. Calculate total pages
        let total_pages = match current_page_size == 0 && current_page > 1 {
            true => current_page - 1,
            false => current_page
        };

        Ok(Json(ProductListResponse {
            products: Vec::new(),
            groups: paginated_groups,
            page: target_page,
            total,
            total_pages,
        }))
    }
}

fn group_products(products: Vec<Product>) -> Vec<(String, Vec<Product>)> {
    let mut group_indices: HashMap<String, usize> = HashMap::new();
    let mut grouped_results: Vec<(String, Vec<Product>)> = Vec::new();

    for product in products {
        match group_indices.get(&product.name) {
            Some(&index) => {
                // Group already exists, push product to the existing group
                grouped_results[index].1.push(product);
            }
            None => {
                // New group encountered, record its index and push to results
                group_indices.insert(product.name.clone(), grouped_results.len());
                grouped_results.push((product.name.clone(), vec![product]));
            }
        }
    }

    grouped_results
}

pub async fn get_section(
    Path(section): Path<String>,
) -> ApiResult<Json<SectionResponse>> {
    let section = Section::from_str(&section).map_err(|err| ApiError::InvalidQuery(err))?;

    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT * FROM products WHERE 1=1"
    );
    builder.push(" AND section = ");
    builder.push_bind(&section);

    let products: Vec<Product> = builder
        .build_query_as::<Product>()
        .fetch_all(get_db_pool())
        .await
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch filters: {}", e)))?;

    let filters = build_filters(&products, section);

    Ok(Json(SectionResponse {
        filters,
        render_specs: section.config().render_specs.clone()
    }))
}

pub async fn get_sections() -> ApiResult<Json<Vec<String>>> {
    Ok(Json(Section::iter().map(|s| s.to_string()).collect::<Vec<String>>()))
}

fn build_filters(products: &[Product], section: Section) -> Vec<FilterOption> {
    let filters = &section.config().filters;
    let mut map = HashMap::new();

    for product in products {
        for filter in filters {
            if let Some(value) = product.specs.get(filter).and_then(|o| o.as_str()) {
                if !value.is_empty() {
                    map
                        .entry(filter)
                        .or_insert_with(std::collections::HashSet::new)
                        .insert(value.to_string());
                }
            }
        }
    }

    let mut options = Vec::new();
    for filter in filters {
        if let Some(values) = map.get(filter) {
            let mut values = values.iter().map(|s| s.clone()).collect::<Vec<String>>();
            values.sort_by(|a, b| {
                match (a.parse::<f64>(), b.parse::<f64>()) {
                    (Ok(na), Ok(nb)) => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
                    _ => a.cmp(b),
                }
            });
            values.push(OTHERS_LABEL.to_string());
            options.push(FilterOption {
                option: filter.clone(),
                values
            });
        }
    }

    options
}

fn apply_filters(builder: &mut QueryBuilder<Postgres>, section: &str, params: &PaginationQuery) {
    builder.push(" AND section = ");
    builder.push_bind(section);

    if let Some(ref search) = params.search {
        builder.push(" AND (title ILIKE ");
        builder.push_bind(format!("%{}%", search));
        builder.push(" OR name ILIKE ");
        builder.push_bind(format!("%{}%", search));
        builder.push(" OR description ILIKE ");
        builder.push_bind(format!("%{}%", search));
        builder.push(")");
    }

    if !params.site.is_empty() {
        builder.push(" AND site IN (");
        let mut separated = builder.separated(", ");
        for site in &params.site {
            separated.push_bind(site);
        }
        builder.push(")");
    }

    if !params.stock.is_empty() {
        builder.push(" AND status IN (");
        let mut separated = builder.separated(", ");
        for stock in &params.stock {
            separated.push_bind(stock);
        }
        builder.push(")");
    }

    // Price range filters
    if let Some(min) = params.min_price {
        builder.push(" AND price >= ");
        builder.push_bind(min as i32);
    }

    if let Some(max) = params.max_price {
        builder.push(" AND price <= ");
        builder.push_bind(max as i32);
    }

    if let Some(specs) = params.specs.clone().and_then(|s| Value::from_str(&s).ok()) {
        if let Ok(specs) = serde_json::from_value::<HashMap<String, Vec<String>>>(specs) {
            for (spec_key, spec_vals) in specs {
                if spec_vals.is_empty() {
                    continue;
                }

                builder.push(" AND (");

                for (i, spec_val) in spec_vals.iter().enumerate() {
                    if i > 0 {
                        builder.push(" OR ");
                    }

                    if spec_val == OTHERS_LABEL {
                        builder.push("(specs IS NULL OR specs->>'");
                        builder.push(&spec_key);
                        builder.push("' IS NULL OR specs->>'");
                        builder.push(&spec_key);
                        builder.push("' = '')");
                    } else {
                        builder.push("specs->>'");
                        builder.push(&spec_key);
                        builder.push("' = ");
                        builder.push_bind(spec_val);
                    }
                }

                builder.push(")");
            }
        }
    }
}