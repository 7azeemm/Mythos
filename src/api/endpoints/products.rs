use crate::api::error::ApiError::InvalidQuery;
use crate::api::error::ApiResult;
use crate::api::filters::{build_all_filters, product_matches_key, FilterGroup};
use crate::storage::PRODUCT_STORAGE;
use crate::utils::serde_ext::JsonExt;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::Site;
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use strum::IntoEnumIterator;

const PAGE_SIZE: usize = 60;

#[derive(Serialize, Debug)]
pub struct PageResponse {
    pub section_info: Option<SectionInfo>,
    pub products: Vec<Product>,
    pub groups: Vec<(String, Vec<Product>)>,
    pub total_products: usize,
    pub total_pages: usize,
    pub page: usize,
}

#[derive(Serialize, Debug)]
pub struct SectionInfo {
    pub filters: Vec<FilterGroup>,
    pub sites: Vec<String>,
    pub components: Vec<String>,
    pub group_by: Vec<String>,
    pub min_price: i32,
    pub max_price: i32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct PageQuery {
    pub require_section_info: Option<bool>,
    pub grouping_mode: bool,
    pub page: Option<usize>,
    pub search: Option<String>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub site: Vec<String>,
    pub stock: Vec<String>,
    pub sort: Option<String>,
    pub filters: Option<String>
}

pub async fn get_products(Path(section): Path<String>, Query(params): Query<PageQuery>) -> ApiResult<Json<PageResponse>> {
    let section = Section::from_str(&section).map_err(|err| InvalidQuery(err))?;
    let page = params.page.unwrap_or(1).max(1);
    let filters = params.filters.as_ref().map(|f| serde_json::from_str::<HashMap<String, Vec<String>>>(f))
        .transpose().map_err(|err| InvalidQuery(err.to_string()))?.unwrap_or_default();

    let mut products = get_filtered_products(section, &params).await;

    // Get Section Details if necessary
    let section_info = match params.require_section_info {
        Some(true) => {
            let (sites, min_price, max_price, filters) = {
                let sites: HashSet<String> = products.iter().map(|p| p.site.clone()).collect();
                let sites: Vec<String> = sites.into_iter().collect();
                let min = products.iter().map(|p| p.price).min().unwrap_or(0);
                let max = products.iter().map(|p| p.price).max().unwrap_or(0);
                let filters = build_all_filters(section, products.as_slice(), &filters);
                (sites, min, max, filters)
            };
            Some(SectionInfo {
                sites,
                filters,
                components: section.config().components.clone(),
                group_by: section.config().group.clone(),
                min_price,
                max_price
            })
        },
        _ => None
    };

    // Apply Filters
    products.retain(|p| filters.iter().all(|(key, ids)| ids.is_empty() || product_matches_key(p, key, ids)));
    let total_products = products.len();

    // Sort Products
    match params.sort.as_deref() {
        Some("price_desc") => products.sort_by(|a, b| b.price.cmp(&a.price)),
        _ => products.sort_by(|a, b| a.price.cmp(&b.price))
    }

    // Return paginated pages or groups if enabled
    let (products, groups, total_pages) = if params.grouping_mode {
        let all_groups = group_products(section, products);
        let total_groups = all_groups.len();
        let total_pages = (total_groups + PAGE_SIZE - 1) / PAGE_SIZE;
        let offset = (page - 1) * PAGE_SIZE;

        let groups: Vec<(String, Vec<Product>)> = all_groups
            .into_iter()
            .skip(offset)
            .take(PAGE_SIZE)
            .collect();

        (Vec::new(), groups, total_pages)
    } else {
        let total_pages = (total_products + PAGE_SIZE - 1) / PAGE_SIZE;
        let offset = (page - 1) * PAGE_SIZE;

        let products: Vec<Product> = products
            .into_iter()
            .skip(offset)
            .take(PAGE_SIZE)
            .collect();

        (products, Vec::new(), total_pages)
    };

    Ok(Json(PageResponse {
        section_info,
        products,
        groups,
        page,
        total_products,
        total_pages,
    }))
}

async fn get_filtered_products(section: Section, params: &PageQuery) -> Vec<Product> {
    PRODUCT_STORAGE.read().await.products
        .iter()
        .filter_map(|(_, p)| {
            // section filter
            if p.section != section {
                return None;
            }

            // search filter
            if let Some(ref search) = params.search {
                let mut s = search.to_lowercase();
                let mut should_match = false;
                if let Some(str) = s.strip_prefix("!=") {
                    should_match = true;
                    s = str.to_string();
                }

                let matches = p.title.to_lowercase().contains(&s)
                    || p.description.as_ref().unwrap_or(&String::default()).to_lowercase().contains(&s);

                if matches == should_match {
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

            Some(p.clone())
        })
        .collect()
}

fn group_products(section: Section, products: Vec<Product>) -> Vec<(String, Vec<Product>)> {
    let mut group_indices: HashMap<String, usize> = HashMap::new();
    let mut groups: Vec<(String, Vec<Product>)> = Vec::new();

    for mut product in products {
        let mut fields: Vec<&str> = Vec::new();
        // for field in &section.config().group {
        //     if let Some(value) = product.specs.get_str(field) {
        //         fields.push(value);
        //     }
        // }

        let name = match fields.len() {
            0 => product.title.clone(),
            _ => fields.join(" | ")
        };

        match group_indices.get(&name) {
            Some(&index) => {
                // Group already exists, push product to the existing group
                groups[index].1.push(product);
            }
            None => {
                // New group encountered, record its index and push to results
                group_indices.insert(name.clone(), groups.len());
                groups.push((name, vec![product]));
            }
        }
    }

    groups
}