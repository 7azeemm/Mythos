use crate::core::product::{Product, ProductStatus};
use crate::core::scanner::CatalogScanner;
use crate::core::sections::Section;
use crate::core::tracking::scan_report::ScanReport;
use crate::utils::file_loader::FileLoader;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use strum::IntoEnumIterator;
use tokio::sync::RwLock;
use tracing::error;

static PRODUCT_STORAGE: Lazy<RwLock<ProductStorage>> = Lazy::new(|| RwLock::new(ProductStorage::default()));
const REMOVED_PRODUCTS_PATH: &str = "data/removed_products.json";

#[derive(Clone, Default)]
pub struct ProductStorage {
    pub products: HashMap<String, Product>,
    pub removed_products: Vec<Product>,
}

#[derive(Clone, Debug)]
pub struct ProductUpdate {
    pub product: Product,
    pub changes: Vec<Value>,
}

#[derive(Clone, Debug, Default)]
pub struct ReparseSummary {
    pub total: usize,
    pub changed: usize,
    pub moved: usize,
    pub updates: Vec<ProductUpdate>,
}

#[derive(Clone, Debug, Default)]
pub struct ProductQuery {
    pub text: Option<String>,
    pub section: Option<Section>,
    pub site: Option<String>,
    pub status: Option<ProductStatus>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub exclude_others: bool,
}

impl ProductStorage {
    pub async fn load() {
        let start_time = Instant::now();
        let mut products = HashMap::new();

        for section in Section::iter() {
            let path = format!("data/{section}.json");
            match FileLoader::load_or_default::<HashMap<String, Product>>(&path).await {
                Ok(section_products) => products.extend(section_products),
                Err(error) => tracing::error!(%error, "Failed to load {section} products"),
            }
        }

        let removed_products = FileLoader::load_or_default::<Vec<Product>>(REMOVED_PRODUCTS_PATH)
            .await
            .unwrap_or_else(|error| {
                tracing::error!(%error, "Failed to load removed products");
                Vec::new()
            });

        let mut storage = PRODUCT_STORAGE.write().await;
        storage.products = products;
        storage.removed_products = removed_products;
        let len = storage.products.len();

        tracing::info!(
            products = len,
            elapsed_ms = start_time.elapsed().as_millis(),
            "Loaded product storage"
        );
    }

    pub async fn save() {
        let start_time = Instant::now();
        let storage = PRODUCT_STORAGE.read().await;
        let mut by_section: HashMap<Section, HashMap<&String, &Product>> = Section::iter()
            .map(|section| (section, HashMap::new()))
            .collect();

        for (id, product) in &storage.products {
            by_section
                .entry(product.section)
                .or_default()
                .insert(id, product);
        }

        for (section, products) in by_section {
            let path = format!("data/{section}.json");
            if let Err(error) = FileLoader::save_to_file(&path, &products).await {
                error!("Failed to save {section} products in {path}: {error}")
            }
        }

        if let Err(error) = FileLoader::save_to_file(REMOVED_PRODUCTS_PATH, &storage.removed_products).await {
            error!("Failed to save removed products in {REMOVED_PRODUCTS_PATH}: {error}")
        }

        tracing::info!(
            elapsed_ms = start_time.elapsed().as_millis(),
            "Saved product storage"
        );
    }

    pub async fn get(product_id: &str) -> Option<Product> {
        PRODUCT_STORAGE
            .read()
            .await
            .products
            .get(product_id)
            .cloned()
    }

    pub async fn search(query: &str, limit: usize) -> Vec<Product> {
        Self::query(
            &ProductQuery {
                text: Some(query.to_string()),
                ..Default::default()
            },
            limit,
        )
        .await
    }

    pub async fn query(query: &ProductQuery, limit: usize) -> Vec<Product> {
        let text = query.text.as_deref().unwrap_or_default().trim().to_lowercase();

        let mut products: Vec<_> = PRODUCT_STORAGE.read().await
            .products
            .values()
            .filter(|product| product_matches_query(product, query, &text))
            .cloned()
            .collect();

        products.sort_by(|a, b| {
            product_match_rank(a, &text)
                .cmp(&product_match_rank(b, &text))
                .then_with(|| a.price.cmp(&b.price))
                .then_with(|| a.title.cmp(&b.title))
        });

        products.truncate(limit);
        products
    }

    pub async fn replace_by_id(original_id: &str, product: Product) -> Result<Product, String> {
        let mut storage = PRODUCT_STORAGE.write().await;
        let previous = storage.products.remove(original_id).ok_or_else(|| "Product not found".to_string())?;

        if product.id != original_id && storage.products.contains_key(&product.id) {
            storage.products.insert(previous.id.clone(), previous);
            return Err("Another product already uses that ID".into());
        }

        let new_id = product.id.clone();
        storage.products.insert(new_id.clone(), product);

        drop(storage);
        Self::save().await;
        Ok(previous)
    }

    pub async fn reparse_section(section: Section) -> Result<ReparseSummary, String> {
        section.try_parser().ok_or_else(|| "The parsers are still starting".to_string())?;
        let mut storage = PRODUCT_STORAGE.write().await;
        let mut summary = ReparseSummary::default();
        let mut affected_sections = HashSet::from([section]);

        for product in storage.products.values_mut().filter(|product| product.section == section) {
            summary.total += 1;
            let old = product.clone();
            let changes = Self::reparse(product);
            if changes.is_empty() {
                continue;
            }

            affected_sections.insert(product.section);
            summary.changed += 1;
            summary.moved += usize::from(product.section != old.section);
            summary.updates.push(ProductUpdate {
                product: product.clone(),
                changes,
            });
        }

        if summary.changed == 0 {
            return Ok(summary);
        }

        Self::save_sections(&storage, &affected_sections).await?;

        Ok(summary)
    }

    pub async fn reparse_product(product_id: &str) -> Result<ProductUpdate, String> {
        let mut storage = PRODUCT_STORAGE.write().await;
        let product = storage.products.get_mut(product_id).ok_or_else(|| "Product not found".to_string())?;
        product
            .section
            .try_parser()
            .ok_or_else(|| "The parsers are still starting".to_string())?;
        let old_section = product.section;
        let changes = Self::reparse(product);
        let update = ProductUpdate {
            product: product.clone(),
            changes,
        };

        if !update.changes.is_empty() {
            let affected_sections = HashSet::from([old_section, update.product.section]);
            Self::save_sections(&storage, &affected_sections).await?;
        }

        Ok(update)
    }

    fn reparse(product: &mut Product) -> Vec<Value> {
        let old = product.clone();
        CatalogScanner::normalize_product_section(product);
        product.filter_ids.clear();
        product.components.clear();
        product.section.parser().parse(product);
        product.record_changes(&old, false)
    }

    async fn save_sections(
        storage: &ProductStorage,
        sections: &HashSet<Section>,
    ) -> Result<(), String> {
        for section in sections {
            let products: HashMap<String, Product> = storage
                .products
                .iter()
                .filter(|(_, product)| product.section == *section)
                .map(|(id, product)| (id.clone(), product.clone()))
                .collect();
            FileLoader::save_to_file(&format!("data/{section}.json"), &products).await?;
        }
        Ok(())
    }

    pub async fn add_note(product_id: &str, note: String) -> Result<ProductUpdate, String> {
        let mut storage = PRODUCT_STORAGE.write().await;
        let product = storage.products.get_mut(product_id).ok_or_else(|| "Product not found".to_string())?;
        let old = product.clone();
        product.notes.push(note);
        let changes = product.record_changes(&old, false);
        let update = ProductUpdate {
            product: product.clone(),
            changes,
        };
        drop(storage);
        Self::save().await;
        Ok(update)
    }

    pub async fn pending_review(section: Option<Section>) -> Vec<Product> {
        let mut products = PRODUCT_STORAGE.read().await
            .products
            .values()
            .filter(|product| !product.approved && section.is_none_or(|section| product.section == section))
            .cloned()
            .collect::<Vec<_>>();
        products.sort_by_key(|product| product.added_at);
        products
    }

    pub async fn approve(product_id: &str) -> Result<Product, String> {
        let mut storage = PRODUCT_STORAGE.write().await;
        let product = storage.products.get_mut(product_id).ok_or_else(|| "Product not found".to_string())?;
        product.approved = true;
        let product = product.clone();
        drop(storage);
        Self::save().await;
        Ok(product)
    }

    /// Moves an active product into the removed-products archive. The move is
    /// rolled back in memory if saving fails.
    pub async fn remove(product_id: &str) -> Result<Product, String> {
        let mut storage = PRODUCT_STORAGE.write().await;
        let mut product = storage.products.remove(product_id).ok_or_else(|| "Product not found".to_string())?;
        product.removed_at = Some(Utc::now());
        storage.removed_products.push(product.clone());

        drop(storage);
        Self::save().await;
        Ok(product)
    }

    pub async fn synchronize(
        mut products: Vec<Product>,
        sections: &[Section],
        sites: &[&'static str],
        report: &mut ScanReport,
    ) {
        let started_at = Instant::now();
        let now = Utc::now();
        let scan_scopes = ScanScopes::from_report(report);
        let mut storage = PRODUCT_STORAGE.write().await;

        let new_product_urls: HashSet<&String> = products.iter().map(|p| &p.url).collect();
        storage.products.retain(|_, existing| {
            let scope = (existing.site.clone(), existing.section);
            let is_in_scope = (sections.is_empty() || sections.contains(&existing.section))
                && (sites.is_empty() || sites.contains(&existing.site.as_str()));
            let missing = is_in_scope && !new_product_urls.contains(&existing.url);
            if !missing {
                return true;
            }

            if !scan_scopes.attempted.contains(&scope) {
                return true;
            }
            if scan_scopes.failed.contains(&scope) {
                return true;
            }

            let mut removed = existing.clone();
            removed.removed_at = Some(now);
            report.removed_items.push(removed);
            false
        });
        storage.removed_products.extend(report.removed_items.iter().cloned());

        let index: HashMap<String, String> = storage
            .products
            .iter()
            .map(|(id, p)| (p.url.clone(), id.clone()))
            .collect();

        for mut product in products {
            let Some(existing) = index.get(&product.url).and_then(|id| storage.products.get_mut(id)) else {
                report.added_items.push(product);
                continue;
            };

            let changes = existing.find_changes(&product, true);

            // Skip if only image changed
            if changes.len() == 1 && existing.image != product.image {
                existing.image = product.image;
                continue
            }

            if !changes.is_empty() {
                product.id = existing.id.clone();
                product.history = existing.history.clone();
                product.notes = existing.notes.clone();
                product.approved = existing.approved;
                product.added_at = existing.added_at;

                let changes = product.record_changes(existing, true);

                report.edited_items.push((product, changes));
            }
        }

        report.update.added = report.added_items.len();
        report.update.edited = report.edited_items.len();
        report.update.removed = report.removed_items.len();
        report.update.duration = started_at.elapsed();

        tracing::info!(
            added = report.update.added,
            edited = report.update.edited,
            removed = report.update.removed,
            elapsed_ms = started_at.elapsed().as_millis(),
            "Synchronized products"
        );
    }

    pub async fn commit(report: &ScanReport) {
        let products = report
            .added_items
            .iter()
            .cloned()
            .chain(
                report
                    .edited_items
                    .iter()
                    .map(|(product, _)| product.clone()),
            )
            .map(|product| (product.id.clone(), product));
        PRODUCT_STORAGE.write().await.products.extend(products);
        Self::save().await
    }

    pub fn get_storage() -> &'static Lazy<RwLock<ProductStorage>> {
        &PRODUCT_STORAGE
    }
}

#[derive(Default)]
struct ScanScopes {
    attempted: HashSet<(String, Section)>,
    failed: HashSet<(String, Section)>,
}

impl ScanScopes {
    fn from_report(report: &ScanReport) -> Self {
        let mut scopes = Self::default();
        for page in report.scrape.iter().flat_map(|section| &section.sites).flat_map(|site| &site.pages) {
            let scope = (page.retailer.clone(), page.section);
            scopes.attempted.insert(scope.clone());
            if page.errors.iter().any(|error| error.error.fails_scope()) {
                scopes.failed.insert(scope);
            }
        }
        scopes
    }
}

fn product_matches_query(product: &Product, query: &ProductQuery, text: &str) -> bool {
    (
        text.is_empty()
        || product.id.to_lowercase().contains(text)
        || product.title.to_lowercase().contains(text)
        || product.description.as_deref().is_some_and(|description| description.to_lowercase().contains(text))
    )
        && query.section.is_none_or(|section| product.section == section)
        && query.site.as_deref().is_none_or(|site| product.site.eq_ignore_ascii_case(site))
        && query.status.as_ref().is_none_or(|status| &product.status == status)
        && query.min_price.is_none_or(|price| product.price >= price)
        && query.max_price.is_none_or(|price| product.price <= price)
        && (!query.exclude_others || product.section != Section::Others)
}

fn product_match_rank(product: &Product, text: &str) -> u8 {
    if text.is_empty() {
        return 5;
    }
    let title = product.title.to_lowercase();
    if product.id.eq_ignore_ascii_case(text) {
        0
    } else if title == text {
        1
    } else if title.starts_with(text) {
        2
    } else if title.contains(text) {
        3
    } else {
        4
    }
}
