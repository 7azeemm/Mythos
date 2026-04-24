use crate::web_scraper::sites::PARSERS;
use crate::web_scraper::specs::{ProductSpecs};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::api::endpoints::info::FilterOption;
use crate::utils::database::get_db_pool;

pub static SPECS_CACHE: Lazy<RwLock<SpecsCache>> = Lazy::new(|| RwLock::new(SpecsCache::default()));

#[derive(Debug, Clone, Default)]
pub struct SpecsCache {
    pub specs: HashMap<String, ProductSpecs>,
    /// data[section][filter_type][filter_value] = [product_ids]
    /// Example: data["pc"]["cpu"]["Intel i9"] = ["prod_1", "prod_2"]
    pub data: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
}

impl SpecsCache {
    pub async fn initialize(&mut self) {
        *self = SpecsCache::default();

        let products: Vec<(String, String, String)> = match sqlx::query_as(
            "SELECT id, section, description FROM products",
        )
            .fetch_all(get_db_pool())
            .await
        {
            Ok(products) => products,
            Err(err) => {
                eprintln!("Failed to retrieve products from database for cache: {err}");
                return;
            }
        };

        for (product_id, product_type, description) in products {
            self.add_product_specs(&product_id, &product_type, &description);
        }

        println!("Specs cache initialized with {} products", self.specs.len());
    }

    fn add_product_specs(&mut self, product_id: &str, product_type: &str, description: &str) {
        let (_, parser) = match PARSERS.iter().find(|(name, _)| name.to_str() == product_type) {
            Some(p) => p,
            None => {
                eprintln!("Unknown product type: {}", product_type);
                return;
            }
        };

        let specs = match parser(description) {
            Ok(specs) => specs,
            Err(err) => {
                eprintln!("Failed to parse product {product_id}: {err}");
                return
            }
        };

        let filters = specs.get_filters();

        let type_entry = self
            .data
            .entry(product_type.to_string())
            .or_insert_with(HashMap::new);

        for (filter_type, filter_value) in filters {
            let filter_entry = type_entry
                .entry(filter_type)
                .or_insert_with(HashMap::new);

            filter_entry
                .entry(filter_value)
                .or_insert_with(Vec::new)
                .push(product_id.to_string());
        }

        self.specs.insert(product_id.to_string(), specs);
    }

    pub fn get_filter_options(&self, product_type: &str) -> HashMap<String, Vec<FilterOption>> {
        let mut filters = HashMap::new();

        if let Some(type_data) = self.data.get(product_type) {
            for (filter_type, filter_values) in type_data {
                let mut options = Vec::new();
                for (filter_value, product_ids) in filter_values {
                    options.push(FilterOption {
                        name: filter_value.to_string(),
                        count: product_ids.len() as i32,
                    });
                }
                options.sort_by(|a, b| b.count.cmp(&a.count));
                filters.insert(filter_type.to_string(), options);
            }
        }

        filters
    }

    pub fn filter_products(&self, product_type: &str, filter_type: &str, filter_value: &str) -> Vec<String> {
        if let Some(type_data) = self.data.get(product_type) {
            if let Some(filter_data) = type_data.get(filter_type) {
                return filter_data.get(filter_value).cloned().unwrap_or_default();
            }
        }
        Vec::new()
    }
}