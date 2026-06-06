use crate::utils::file_loader::FileLoader;
use crate::web_scraper::errors::{CycleReport, KnownEvents, ParseError, ParseErrorKind, SectionReport};
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::{Section, SectionConfig, SECTION_PARSERS};
use crate::web_scraper::sites::{get_site_from_str, ProductDescription, Site, DESCRIPTION_CACHE, PAGE_CACHE, SITES};
use crate::web_scraper::updater::ProductUpdater;
use chrono::Utc;
use futures::future::join_all;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use serde_json::Value;
use strum::IntoEnumIterator;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::web_scraper::sites::megapc::MegaPC;

static PRODUCT_MANAGER: OnceLock<Arc<ProductManager>> = OnceLock::new();

pub struct ProductManager {
    report: Mutex<CycleReport>,
    known_events: Mutex<KnownEvents>
}

impl ProductManager {
    pub fn get() -> Arc<Self> {
        PRODUCT_MANAGER.get().cloned().unwrap()
    }

    pub async fn schedule() {
        tokio::spawn(async move {
            *PAGE_CACHE.write().await = FileLoader::load_or_create::<HashMap<String, Vec<Product>>>("pages_cache.json").await.unwrap();
            *DESCRIPTION_CACHE.write().await = FileLoader::load_or_create::<HashMap<String, ProductDescription>>("descriptions.json").await.unwrap();
            
            // {
            //     let mut page_cache = PAGE_CACHE.write().await;
            //     let sections = vec![Section::Laptop, Section::GamingLaptop, Section::MacBook, Section::Mouse, Section::Keyboard, Section::AccessoriesCombo];
            //     let mut urls = Vec::new();
            //     for (url, products) in page_cache.iter_mut() {
            //         if products.iter().any(|p| sections.contains(&p.section)) {
            //             urls.push(url.clone());
            //         }
            //     }
            //     page_cache.retain(|url, _| !urls.contains(url));
            //     page_cache.retain(|_, products| !products.is_empty());
            //     FileLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &page_cache).await.unwrap();
            //     println!("Done");
            //     return;
            // }

            SectionConfig::load().await.expect("Failed to load section configs");
            let manager = Arc::new(ProductManager::new().await.expect("Failed to create Product Manager"));
            PRODUCT_MANAGER.set(manager.clone()).map_err(|e| "Failed to set ProductManager".to_string()).unwrap();

            loop {
                manager.run().await;

                {
                    let file_name = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                    let path = format!("reports/{file_name}.json");
                    let mut report = manager.report.lock().await;
                    if let Err(err) = FileLoader::save_to_file::<CycleReport>(&path, &report).await {
                        eprintln!("Failed to save cycle report: {err}")
                    }

                    *report = CycleReport::new()
                }

                FileLoader::save_to_file::<KnownEvents>("known_events.json", &*manager.known_events.lock().await).await.unwrap();
                FileLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &*PAGE_CACHE.read().await).await.unwrap();
                FileLoader::save_to_file::<HashMap<String, ProductDescription>>("descriptions.json", &*DESCRIPTION_CACHE.read().await).await.unwrap();

                sleep(Duration::from_secs(3600)).await;
            }
        });
    }

    async fn new() -> Result<Self, Box<dyn Error>> {
        // let known_events = FileLoader::load_or_create::<KnownEvents>("known_events.json").await?;
        let known_events = KnownEvents::default();

        Ok(Self {
            report: Mutex::new(CycleReport::new()),
            known_events: Mutex::new(known_events)
        })
    }

    async fn run(&self) {
        // let specific_sites = Some(vec![MegaPC {}.name()]);
        let sections = vec![Section::Laptop, Section::GamingLaptop];
        // let sections = Section::list();
        let start_time = Instant::now();
        self.report.lock().await.started_at = Utc::now();

        let products = self.fetch_sites(sections.clone(), &None).await;
        // let products = self.fetch_sites(sections).await;

        let update_start_time = Instant::now();
        let mut products = {
            let mut report = self.report.lock().await;
            ProductUpdater::archive_missing_products(&mut *report, &vec![], &None, &None).await;
            // ProductUpdater::sync(&mut *report, products).await
            products
        };

        println!("---");

        let parse_start_time = Instant::now();

        for mut product in &mut products {
            self.parse(&mut product).await;
        }

        println!("Parsed {} products in {:.2?}", products.len(), parse_start_time.elapsed());
        println!("---");

        let mut report = self.report.lock().await;
        for e in report.parse.iter() {
            if sections.contains(&e.product.section) {
                if e.error != ParseErrorKind::NoSectionMatched {
                    if e.error == ParseErrorKind::NotInDataset {
                        // println!("NotInDataset: {}", e.product.name);
                    } else {
                        // println!("{:#?}", e);
                    }
                } else {
                    // println!("{:#?}", e);
                }
            }
        }

        ProductUpdater::insert_products(&mut *report, products).await;

        for error in report.update.errors.iter() {
            self.known_events.lock().await.update_error(error.clone());
        }
        report.update.duration = update_start_time.elapsed();

        report.completed_at = Utc::now();
        report.duration = start_time.elapsed();

        println!("Done");
    }

    async fn fetch_sites(&self, sections: Vec<Section>, specific_sites: &Option<Vec<&'static str>>) -> Vec<Product> {
        let start_time = Instant::now();
        let mut all_products = Vec::new();

        // 1. Fetch Sites
        for section in sections {
            let sites: Vec<_> = SITES.iter().filter(|site|
                specific_sites.as_ref().map_or(true, |sites_list| sites_list.contains(&site.name()))
            ).collect();

            println!("Started scrapping `{:?}` section from {} sites", section, sites.len());
            let start_time = Instant::now();
            let mut tasks = Vec::new();

            for site in sites {
                for (_, url) in site.config().sections.iter().filter(|(s, _)| *s == section) {
                    if specific_sites.is_some() {
                        PAGE_CACHE.write().await.remove(*url);
                    }
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
        let old_count = all_products.len();
        let mut map: HashMap<String, Product> = HashMap::with_capacity(all_products.len());
        for product in all_products {
            match map.get_mut(&product.url) {
                Some(existing) => if !product.section.is_low_priority() && existing.section.is_low_priority() {
                    *existing = product;
                }
                None => {
                    map.insert(product.url.clone(), product);
                }
            }
        }
        all_products = map.into_values().collect();
        println!("Removed {} Duplicated Products", old_count - all_products.len());

        println!("Successfully Fetched {} products in {:.2?}", all_products.len(), start_time.elapsed());

        all_products
    }

    async fn parse(&self, product: &mut Product) {
        if Section::Trash.parser().matches(&product.title, &product.description, false) {
            product.section = Section::Trash;
            return;
        }

        let parser = product.section.parser();
        let config = product.section.config();

        if config.unswitchable || parser.matches(&product.title, &product.description, config.skip_include_check) {
            if let Err(err) = parser.parse(product) {
                self.add_parse_error(err, product.clone()).await;
            }
            return;
        }

        let switchable_to = &config.switchable_to;
        for section in Section::list() {
            if section == product.section || (!switchable_to.is_empty() && !switchable_to.contains(&section.to_string())) {
                continue
            }

            let parser = section.parser();
            if parser.matches(&product.title, &product.description, false) {
                // println!("MOVED FROM {} TO {}: {}", product.section, section, product.title);
                product.section = section;
                if let Err(err) = parser.parse(product) {
                    self.add_parse_error(err, product.clone()).await;
                }
                return;
            }
        }

        // println!("NO MATCH: {}", product.title);

        product.section = Section::Trash;
        self.add_parse_error(ParseErrorKind::NoSectionMatched, product.clone()).await;
        return;
    }

    async fn add_parse_error(&self, error: ParseErrorKind, product: Product) {
        let parse_error = ParseError { error, product, timestamp: Utc::now() };
        self.report.lock().await.parse.push(parse_error.clone());
        self.known_events.lock().await.parse_error(parse_error);
    }
}