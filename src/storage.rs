use crate::utils::file_loader::FileLoader;
use crate::web_scraper::manager::ProductManager;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use strum::IntoEnumIterator;
use tokio::sync::RwLock;

pub static PRODUCT_STORAGE: Lazy<RwLock<ProductStorage>> = Lazy::new(|| RwLock::new(ProductStorage::default()));

#[derive(Default)]
pub struct ProductStorage {
    pub products: HashMap<String, Product>,
    pub removed_products: Vec<Product>
}

impl ProductStorage {
    pub async fn load() {
        let start_time = Instant::now();
        let mut storage = PRODUCT_STORAGE.write().await;
        for section in Section::iter() {
            let section_id = section.to_string();
            let path = format!("data/{section_id}.json");
            // match FileLoader::load_or_default::<HashMap<String, Product>>(&path).await {
            //     Ok(data) => storage.products.extend(data),
            //     Err(err) => eprintln!("Failed to save `{section_id}` products: {err}")
            // }
        }
        println!("Loaded Products in {:.2?}", start_time.elapsed());
    }

    async fn save() {
        let start_time = Instant::now();
        let storage = PRODUCT_STORAGE.read().await;

        // 1. Group products by section
        let mut by_section: HashMap<Section, HashMap<&String, &Product>> = HashMap::new();
        for (id, product) in storage.products.iter() {
            by_section.entry(product.section).or_default().insert(id, product);
        }

        // 2. Write each section to its file
        for (section, data) in by_section {
            let path = format!("data/{}.json", section.to_string());
            if let Err(err) = FileLoader::save_to_file(&path, &data).await {
                eprintln!("Failed to save `{section}` products: {err}");
            }
        }

        println!("Saved Products in {:.2?}", start_time.elapsed());
    }

    pub async fn insert(products: Vec<Product>) {
        let products: HashMap<String, Product> = products.into_iter().map(|p| (p.id.clone(), p)).collect();
        PRODUCT_STORAGE.write().await.products.extend(products);

        Self::save().await;
    }

    pub async fn update(products: Vec<Product>, sections: &[Section], sites: &[&'static str]) -> Vec<Product> {
        let start_time = Instant::now();
        let now = Utc::now();
        let mut storage = PRODUCT_STORAGE.write().await;

        // 1. Remove missing products
        let new_product_ids: HashSet<&String> = products.iter().map(|p| &p.id).collect();
        let mut removed_products = Vec::new();
        storage.products.retain(|id, p| {
            let section_match = sections.is_empty() || sections.contains(&p.section);
            let site_match = sites.is_empty() || sites.contains(&p.site.as_str());

            let should_keep = if section_match && site_match {
                new_product_ids.contains(id)
            } else {
                true
            };

            if !should_keep {
                let mut product = p.clone();
                product.removed_at = Some(now.clone());
                removed_products.push(product);
            }

            should_keep
        });
        println!("Removed {} products", removed_products.len());
        storage.removed_products = removed_products;

        // report.update.removed += to_archive.len();
        // report.removed_items.extend(to_archive);

        // 2. Add new products and check for changes
        let mut added_products = Vec::new();
        for mut product in products {
            let Some(existing) = storage.products.get_mut(&product.id) else {
                storage.products.insert(product.id.clone(), product.clone());
                added_products.push(product);
                continue
            };

            let title_changed = product.title != existing.title;
            let desc_changed = product.description != existing.description;
            let image_changed = product.image != existing.image;
            let status_changed = product.status != existing.status;
            let price_changed = product.price != existing.price;
            let old_price_changed = product.old_price != existing.old_price;

            // Changed Product
            if title_changed | desc_changed | image_changed | status_changed | price_changed | old_price_changed {
                fn push_change<T: Serialize>(history: &mut Vec<Value>, field: &str, old: T, new: T) {
                    history.push(json!({
                        "field": field,
                        "old_value": old,
                        "new_value": new,
                        "timestamp": Utc::now()
                    }));
                }

                let mut changes = Vec::new();

                if title_changed { push_change(&mut changes, "title", &product.title, &existing.title); }
                if desc_changed { push_change(&mut changes, "description", &product.description, &existing.description); }
                if image_changed { push_change(&mut changes, "image", &product.image, &existing.image); }
                if status_changed { push_change(&mut changes, "status", &product.status, &existing.status); }
                if price_changed { push_change(&mut changes, "price", &product.price, &existing.price); }
                if old_price_changed { push_change(&mut changes, "old_price", &product.old_price, &existing.old_price); }

                // report.update.edited += 1;
                // report.edited_items.push((product.clone(), changes.clone()));

                let mut history = existing.history.as_array().cloned().unwrap_or_default();
                history.extend(changes);

                product.history = Value::Array(history);
                product.updated_at = Some(now.clone());
                product.added_at = existing.added_at;

                *existing = product;
            }
        }

        let manager = ProductManager::get();
        let mut report = manager.report.lock().await;
        report.update.added += added_products.len();
        report.added_items.extend(added_products.clone());

        println!("Synced products in {:.2?}", start_time.elapsed());

        added_products
    }
}