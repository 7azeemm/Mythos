use crate::api::endpoints::info::FilterOption;
use crate::web_scraper::product::Product;
use crate::web_scraper::specs::ProductSpecs;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub static SPECS_CACHE: Lazy<RwLock<SpecsCache>> = Lazy::new(|| RwLock::new(SpecsCache::default()));

#[derive(Debug, Clone, Default)]
pub struct SpecsCache {
    /// data[section][option][value] = [product_ids]
    /// Example: data["pc"]["cpu"]["Intel i9"] = ["prod_1", "prod_2"]
    pub data: HashMap<String, HashMap<String, HashMap<String, Vec<String>>>>,
    pub specs: HashMap<String, ProductSpecs>,
}

impl SpecsCache {
    pub async fn initialize(&mut self, products: Vec<Product>) {
        *self = SpecsCache::default();

        for product in products {
            // self.add_product(&product.id, &product.section, &product.description);
        }

        println!("Specs cache initialized with {} products", self.specs.len());
    }

    fn add_product(&mut self, product_id: &str, section: &str, description: &str) {
        // let parser = match Section::from_str(section) {
        //     Some(section) => section.parser(),
        //     None => {
        //         eprintln!("Unknown section: {}", section);
        //         return;
        //     }
        // };
        //
        // let specs = match parser(description) {
        //     Ok(specs) => specs,
        //     Err(err) => {
        //         eprintln!("Failed to parse product {product_id}: {err}");
        //         return
        //     }
        // };
        //
        // let section_entry = self
        //     .data
        //     .entry(section.to_string())
        //     .or_insert_with(HashMap::new);
        //
        // for (option, value) in specs.get_filters() {
        //     let option_entry = section_entry
        //         .entry(option)
        //         .or_insert_with(HashMap::new);
        //
        //     option_entry
        //         .entry(value)
        //         .or_insert_with(Vec::new)
        //         .push(product_id.to_string());
        // }
        //
        // self.specs.insert(product_id.to_string(), specs);
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