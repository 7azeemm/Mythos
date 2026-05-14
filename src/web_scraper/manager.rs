use crate::utils::json_loader::JsonLoader;
use crate::web_scraper::errors::{CycleReport, KnownEvents, ParseError, ParseErrorKind, SectionReport};
use crate::web_scraper::parsers::{GenericSectionParser, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, PAGE_CACHE, SITES};
use crate::web_scraper::updater::ProductUpdater;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;

pub struct ProductManager {
    parsers: HashMap<Section, Arc<dyn SectionParser>>,
    report: Mutex<CycleReport>,
    known_events: Mutex<KnownEvents>
}

impl ProductManager {
    pub async fn schedule() {
        tokio::spawn(async move {
            *PAGE_CACHE.write().await = JsonLoader::load_or_create_default::<HashMap<String, Vec<Product>>>("pages_cache.json").await.unwrap();
            *DESCRIPTION_CACHE.write().await = JsonLoader::load_or_create_default::<HashMap<String, ProductDescription>>("descriptions.json").await.unwrap();
            
            let manager = ProductManager::new().await.expect("Failed to create Product Manager");
            
            loop {
                manager.run().await;

                {
                    let file_name = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                    let path = format!("reports/{file_name}.json");
                    let mut report = manager.report.lock().await;
                    if let Err(err) = JsonLoader::save_to_file::<CycleReport>(&path, &report).await {
                        eprintln!("Failed to save cycle report: {err}")
                    }

                    *report = CycleReport::new()
                }

                JsonLoader::save_to_file::<KnownEvents>("known_events.json", &*manager.known_events.lock().await).await.unwrap();
                JsonLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &*PAGE_CACHE.read().await).await.unwrap();
                JsonLoader::save_to_file::<HashMap<String, ProductDescription>>("descriptions.json", &*DESCRIPTION_CACHE.read().await).await.unwrap();

                sleep(Duration::from_secs(3600)).await;
            }
        });
    }
    
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let mut parsers: HashMap<Section, Arc<dyn SectionParser>> = HashMap::new();
        let known_events = JsonLoader::load_or_create_default::<KnownEvents>("known_events.json").await?;
        let section_configs = JsonLoader::load_or_create_default::<Vec<SectionConfig>>("config/sections.json").await?;

        for config in section_configs {
            let section_id = config.id.clone();
            let section = Section::from_str(&section_id)
                .ok_or(format!("Section enum does not have variant {section_id}"))?;

            let parser: Arc<dyn SectionParser> = {
                let dataset = match config.use_dataset {
                    true => JsonLoader::load_from_file(&format!("config/datasets/{section_id}.json")).await?,
                    false => None
                };

                Arc::new(GenericSectionParser { config, dataset })
            };

            parsers.insert(section, parser);
        }

        Ok(Self {
            parsers,
            report: Mutex::new(CycleReport::new()),
            known_events: Mutex::new(known_events)
        })
    }

    async fn run(&self) {
        let sections = vec![Section::CPU];
        let start_time = Instant::now();
        self.report.lock().await.started_at = Utc::now();

        let products = self.fetch_sites(sections).await;

        let update_start_time = Instant::now();
        let new_products = {
            let mut report = self.report.lock().await;
            ProductUpdater::archive_missing_products(&mut *report, &products).await;
            ProductUpdater::sync(&mut *report, products).await
        };

        let mut parsed_products = Vec::new();
        for product in new_products {
            if let Some(product) = self.parse(product).await {
                parsed_products.push(product);
            }
        }

        let mut report = self.report.lock().await;

        ProductUpdater::insert_products(&mut *report, parsed_products).await;

        for error in report.update.errors.iter() {
            self.known_events.lock().await.update_error(error.clone());
        }
        report.update.duration = update_start_time.elapsed();

        report.completed_at = Utc::now();
        report.duration = start_time.elapsed();
    }

    async fn fetch_sites(&self, sections: Vec<Section>) -> Vec<Product> {
        let start_time = Instant::now();
        let mut all_products = Vec::new();

        // 1. Fetch Sites
        for section in sections {
            println!("Started scrapping `{:?}` section from {} sites", section, SITES.len());
            let start_time = Instant::now();
            let mut tasks = Vec::new();

            for site in SITES.iter() {
                for (_, url) in site.config().sections.iter().filter(|(s, _)| *s == section) {
                    tasks.push(async move {
                        site.scrape(&url, section).await
                    });
                }
            }

            let mut known_events = self.known_events.lock().await;
            let mut sites = Vec::new();
            let mut total_products = 0;
            for (site_report, products) in join_all(tasks).await {
                for page_report in site_report.pages.iter() {
                    for error in page_report.errors.iter() {
                        known_events.scrape_error(error.clone());
                    }
                }
                sites.push(site_report);
                total_products += products.len();
                all_products.extend(products);
            }

            self.report.lock().await.scrape.push(SectionReport {
                section,
                total_products,
                sites,
                duration: start_time.elapsed()
            })
        }

        // 2. Remove Duplicated Products
        all_products.sort_unstable_by(|a, b| a.url.cmp(&b.url));
        all_products.dedup_by(|a, b| a.url == b.url);

        println!("Successfully Fetched {} products in {:.2?}", all_products.len(), start_time.elapsed());

        all_products
    }

    async fn parse(&self, mut product: Product) -> Option<Product> {
        let mut handled_sections: Vec<Section> = Vec::new();

        for section in product.sections.clone() {
            if handled_sections.contains(&section) {
                continue
            }

            let section = match section.parent() {
                Some(parent) => {
                    handled_sections.push(section);
                    parent
                },
                None => section
            };

            handled_sections.push(section);

            let parser = self.parsers.get(&section).cloned().unwrap();
            if !parser.matches_keywords(&product.title) {
                product.sections.retain(|s| {
                    !section.children().contains(s) && s != &section
                });
            } else {
                product = match self.try_parse(product, section, &parser).await {
                    Some(p) => p,
                    None => return None
                };
            }
        }

        if !product.sections.is_empty() {
            return Some(product)
        }

        // Try other sections
        for (section, parser) in self.parsers.iter() {
            if handled_sections.contains(&section) {
                continue;
            }

            if parser.matches_keywords(&product.title) {
                return self.try_parse(product, *section, &parser).await
            }
        }

        // No match
        self.add_parse_error(ParseErrorKind::NoSectionMatched, product).await;

        None
    }

    async fn try_parse(&self, mut product: Product, section: Section, parser: &Arc<dyn SectionParser>) -> Option<Product> {
        if !product.sections.contains(&section) {
            product.sections.push(section);
        }
        if let Some(subsection) = parser.detect_subsection(&product.title) {
            if !product.sections.contains(&subsection) {
                product.sections.push(subsection);
            }
        }

        product.name = parser.clean_title(&product.title);
        parser.parse_specs(&mut product.specs, &product.name, &product.description);

        match parser.lookup_dataset(&product.name, &mut product.specs) {
            Some(id) => product.id = id,
            None => {
                self.add_parse_error(ParseErrorKind::NotInDataset, product).await;
                return None
            }
        }

        if let Err(error) = parser.validate_required_fields(&mut product.specs) {
            self.add_parse_error(error, product).await;
            return None
        }
        
        Some(product)
    }

    async fn add_parse_error(&self, error: ParseErrorKind, product: Product) {
        let parse_error = ParseError { error, product, timestamp: Utc::now() };
        self.report.lock().await.parse.push(parse_error.clone());
        self.known_events.lock().await.parse_error(parse_error);
    }
}

async fn save_known_events(known_events: &KnownEvents) {
    let data = serde_json::to_string_pretty(known_events).unwrap();
    tokio::fs::write("known_events.json", data).await.unwrap();
}

static DESCRIPTION_CACHE: Lazy<RwLock<HashMap<String, ProductDescription>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Deserialize, Serialize)]
pub struct ProductDescription {
    pub description: String,
    pub timestamp: DateTime<Utc>
}

async fn load_descriptions() {
    let Ok(data) = tokio::fs::read_to_string("descriptions.json").await else {
        return;
    };

    let descriptions: HashMap<String, ProductDescription> = serde_json::from_str(&data)
        .expect("Description Cache initialization failed");

    *DESCRIPTION_CACHE.write().await = descriptions;
}

async fn save_descriptions() {
    let cache = DESCRIPTION_CACHE.read().await;
    let data = serde_json::to_string_pretty(&*cache).unwrap();
    tokio::fs::write("descriptions.json", data).await.unwrap();
}

pub async fn get_product_description(url: &str) -> Option<String> {
    if let Some(cached) = DESCRIPTION_CACHE.read().await.get(url) {
        let duration = Utc::now().signed_duration_since(cached.timestamp);
        if duration > chrono::Duration::days(7) {
            DESCRIPTION_CACHE.write().await.remove(url);
            return None;
        }
        return Some(cached.description.clone());
    }
    None
}

pub async fn add_product_description(url: String, description: String) {
    DESCRIPTION_CACHE.write().await.insert(url, ProductDescription {
        description,
        timestamp: Utc::now()
    });
}