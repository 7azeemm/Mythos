use crate::api::error::{ApiError, ApiResult};
use crate::storage::{ProductStorage, PRODUCT_STORAGE};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;
use strum::IntoEnumIterator;
use crate::web_scraper::sites::mytek::Mytek;
use crate::web_scraper::sites::Site;

const PAGE_SIZE: usize = 60;
pub const OTHERS_LABEL: &str = "Others";

#[derive(Serialize, Debug)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub groups: Vec<(String, Vec<Product>)>,
    pub total: usize,
    pub total_pages: usize,
    pub page: usize,
}

//FIXME: sites are not sent
#[derive(Serialize, Debug)]
pub struct SectionResponse {
    pub filters: Vec<FilterOption>,
    pub render_specs: Vec<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct FilterOption {
    pub option: String,
    pub values: Vec<String>
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PaginationQuery {
    pub page: Option<usize>,
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
    let start_time = Instant::now();
    let section = Section::from_str(&section).map_err(|err| ApiError::InvalidQuery(err))?;
    let page = params.page.unwrap_or(1).max(1);

    let mut products = get_filtered_products(section, &params).await;

    // 2. Sort
    match params.sort.as_deref() {
        Some("price_desc") => products.sort_by(|a, b| b.price.cmp(&a.price)),
        _ => products.sort_by(|a, b| a.price.cmp(&b.price))
    }

    let total = products.len();

    let response = if !params.groups {
        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let offset = (page - 1) * PAGE_SIZE;

        let products: Vec<Product> = products
            .into_iter()
            .skip(offset)
            .take(PAGE_SIZE)
            .collect();

        ProductListResponse {
            products,
            groups: Vec::new(),
            page,
            total,
            total_pages,
        }
    } else {
        let groups = {
            let mut group_indices: HashMap<String, usize> = HashMap::new();
            let mut groups: Vec<(String, Vec<Product>)> = Vec::new();

            for product in products {
                match group_indices.get(&product.name) {
                    Some(&index) => {
                        // Group already exists, push product to the existing group
                        groups[index].1.push(product);
                    }
                    None => {
                        // New group encountered, record its index and push to results
                        group_indices.insert(product.name.clone(), groups.len());
                        groups.push((product.name.clone(), vec![product]));
                    }
                }
            }

            groups
        };

        let mut paginated_groups = Vec::new();
        let mut current_page = 1;
        let mut current_page_size = 0;

        for (group_name, group_products) in groups {
            let group_size = group_products.len();

            if current_page == page {
                paginated_groups.push((group_name, group_products));
            }

            current_page_size += group_size;

            if current_page_size >= PAGE_SIZE {
                current_page += 1;
                current_page_size = 0;
            }
        }

        let total_pages = if current_page_size == 0 && current_page > 1 {
            current_page - 1
        } else {
            current_page
        };

        ProductListResponse {
            products: Vec::new(),
            groups: paginated_groups,
            page,
            total,
            total_pages,
        }
    };

    println!("Processed request in {:.2?}", start_time.elapsed());

    Ok(Json(response))
}

async fn get_filtered_products(section: Section, params: &PaginationQuery) -> Vec<Product> {
    let specs = params.specs.as_ref()
        .map(|s| match serde_json::from_str::<HashMap<String, Vec<String>>>(s) {
            Ok(specs) => Some(specs),
            Err(err) => {
                eprintln!("Failed to decode specs `{s}`: {err}");
                None
            }
        }).flatten();

    PRODUCT_STORAGE.read().await.products
        .iter()
        .filter_map(|(_, p)| {
            // section filter
            if p.section != section {
                return None;
            }

            // search filter
            if let Some(ref search) = params.search {
                let s = search.to_lowercase();
                let matches = p.title.to_lowercase().contains(&s)
                    || p.description.as_ref().unwrap_or(&String::default()).to_lowercase().contains(&s);

                if !matches {
                    return None;
                }
            }

            // site filter
            if !params.site.is_empty() && !params.site.contains(&p.site.as_str().to_string()) {
                return None;
            }

            // stock filter
            if !params.stock.is_empty() && !params.stock.contains(&p.status.to_string()) {
                return None;
            }

            // price filters
            if let Some(min) = params.min_price {
                if p.price < min as i32 {
                    return None;
                }
            }

            if let Some(max) = params.max_price {
                if p.price > max as i32 {
                    return None;
                }
            }

            if let Some(specs) = &specs {
                let product_specs = &p.specs;

                for (spec_key, spec_vals) in specs {
                    if spec_vals.is_empty() {
                        continue;
                    }

                    let prod_vals = product_specs.get(spec_key).and_then(|v| v.as_str());

                    let ok = spec_vals.iter().any(|v| {
                        if v == OTHERS_LABEL {
                            // match missing or empty
                            prod_vals.map(|val| val.is_empty()).unwrap_or(true)
                        } else {
                            // normal match
                            prod_vals.map(|val| val == v).unwrap_or(false)
                        }
                    });

                    if !ok {
                        return None;
                    }
                }
            }

            Some(p.clone())
        })
        .collect()
}

pub async fn get_section(Path(section): Path<String>) -> ApiResult<Json<SectionResponse>> {
    let section = Section::from_str(&section).map_err(|err| ApiError::InvalidQuery(err))?;
    Ok(Json(SectionResponse {
        filters: ProductStorage::get_filters(section).await,
        render_specs: section.config().render_specs.clone()
    }))
}

pub async fn get_sections() -> ApiResult<Json<Vec<String>>> {
    Ok(Json(Section::iter().map(|s| s.to_string()).collect::<Vec<String>>()))
}