use crate::storage::ProductStorage;
use crate::utils::file_loader::FileLoader;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::errors::{CycleReport, KnownEvents, ParseError, ParseErrorKind};
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{ProductDescription, Site, DESCRIPTION_CACHE, PAGE_CACHE, SITES};
use chrono::Utc;
use futures::{stream, StreamExt};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::utils::regex_cache::RegexCache;

static PRODUCT_MANAGER: OnceLock<Arc<ProductManager>> = OnceLock::new();

pub struct ProductManager {
    pub report: Mutex<CycleReport>,
    known_events: Mutex<KnownEvents>
}

impl ProductManager {
    pub fn get() -> Arc<Self> {
        PRODUCT_MANAGER.get().cloned().unwrap()
    }

    pub async fn schedule() {
        tokio::spawn(async move {
            *PAGE_CACHE.write().await = FileLoader::load_or_default::<HashMap<String, Vec<Product>>>("pages_cache.json").await.unwrap();
            *DESCRIPTION_CACHE.write().await = FileLoader::load_or_default::<HashMap<String, ProductDescription>>("descriptions.json").await.unwrap();
            
            {
                let mut page_cache = PAGE_CACHE.write().await;
                let sections = vec![];
                let mut urls = Vec::new();
                for (url, products) in page_cache.iter_mut() {
                    if products.iter().any(|p| sections.contains(&p.section)) {
                        urls.push(url.clone());
                    }
                }
                page_cache.retain(|url, _| !urls.contains(url));
                // page_cache.retain(|_, products| !products.is_empty());
                // for (_, products) in page_cache.iter_mut() {
                //     for product in products {
                //         if product.section == GamingLaptop && product.site == "SBSInformatique" {
                //             product.section = Laptop;
                //         }
                //     }
                // }
                FileLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &page_cache).await.unwrap();
            }

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
        let start_time = Instant::now();
        self.report.lock().await.started_at = Utc::now();

        let sites = &[];
        // let sections = Section::iter().collect::<Vec<_>>();
        let sections = vec![Section::CPU, Section::GPU, Section::GamingPC, Section::Laptop, Section::GamingLaptop];

        let mut products = self.fetch_sites(&sections, sites).await;
        products.retain(|p| sections.contains(&p.section));
        let mut products = ProductStorage::update(products, &sections, sites).await;
        self.parse(&mut products).await;
        ProductStorage::insert(products).await;

        println!("Update Cycle took {:.2?}", start_time.elapsed());
    }

    async fn fetch_sites(&self, sections: &[Section], sites: &[&'static str]) -> Vec<Product> {
        let start_time = Instant::now();
        let mut all_products = Vec::new();

        // Cache
        if let products = PAGE_CACHE.read().await.values() {
            for products in products {
                all_products.extend(products.clone());
            }
            return all_products;
        }

        let mut browser_tasks = Vec::new();
        let mut client_tasks = Vec::new();

        // Collect Pages
        for section in sections {
            for site in SITES.iter().filter(|s| sites.is_empty() || sites.contains(&s.name())).collect::<Vec<_>>() {
                for (_, url) in site.config().sections.iter().filter(|(s, _)| s == section) {
                    let task = async move {
                        let (reports, products, page_count) = site.scrape_page(url, 1, *section).await;
                        let page_count = page_count.unwrap_or(1);
                        let mut tasks = Vec::new();
                        for page in 2..page_count+1 {
                            tasks.push(async move {
                                let (reports, products, _) = site.scrape_page(url, page, *section).await;
                                sleep(Duration::from_millis(100)).await;
                                (reports, products)
                            });
                        }
                        sleep(Duration::from_millis(100)).await;
                        (reports, products, tasks)
                    };

                    match site.config().web_client_type {
                        WebClientType::HttpClient => client_tasks.push(task),
                        WebClientType::Browser => browser_tasks.push(task)
                    };
                }
            }
        }

        // Fetch the first pages
        let process_first_pages = |tasks: Vec<_>, buffer_size: usize| async move {
            stream::iter(tasks)
                .buffer_unordered(buffer_size)
                .collect::<Vec<_>>()
                .await
        };

        let (browser_future, client_future) = {
            let mut rng = rand::rng();
            browser_tasks.shuffle(&mut rng);
            client_tasks.shuffle(&mut rng);
            (process_first_pages(browser_tasks, 3), process_first_pages(client_tasks, 10))
        };

        let (browser_results, client_results) = tokio::join!(browser_future, client_future);

        let mut page_reports = Vec::new();
        let mut client_tasks = Vec::new();
        let mut browser_tasks = Vec::new();

        for (reports, products, tasks) in browser_results {
            page_reports.push(reports);
            all_products.extend(products);
            browser_tasks.extend(tasks);
        }

        for (reports, products, tasks) in client_results {
            page_reports.push(reports);
            all_products.extend(products);
            client_tasks.extend(tasks);
        }

        // Fetch the rest
        let process_subsequent_pages = |tasks: Vec<_>, buffer_size: usize| async move {
            stream::iter(tasks)
                .buffer_unordered(buffer_size)
                .collect::<Vec<_>>()
                .await
        };

        let (browser_future, client_future) = {
            let mut rng = rand::rng();
            browser_tasks.shuffle(&mut rng);
            client_tasks.shuffle(&mut rng);
            (process_subsequent_pages(browser_tasks, 3), process_subsequent_pages(client_tasks, 10))
        };

        let (browser_results, client_results) = tokio::join!(browser_future, client_future);

        for (reports, products) in browser_results.into_iter().chain(client_results) {
            page_reports.push(reports);
            all_products.extend(products);
        }

        let mut known_events = self.known_events.lock().await;
        for report in page_reports {
            for error in report.errors {
                known_events.scrape_error(error);
            }
        }

        // Remove Duplicated Products
        let old_count = all_products.len();
        let mut map: HashMap<String, Product> = HashMap::with_capacity(all_products.len());
        for product in all_products {
            match map.get_mut(&product.id) {
                Some(existing) => if !product.section.is_low_priority() && existing.section.is_low_priority() {
                    *existing = product;
                }
                None => {
                    map.insert(product.id.clone(), product);
                }
            }
        }
        all_products = map.into_values().collect();
        println!("Removed {} Duplicated Products", old_count - all_products.len());

        println!("Successfully Fetched {} products in {:.2?}", all_products.len(), start_time.elapsed());

        all_products
    }

    fn verify_section(&self, product: &mut Product) {
        // store count of moved/fixed section products for statistics
        product.components.insert("original_section".to_string(), product.section.to_string());

        if product.price == 0 {
            product.section = Section::Others;
            return;
        }

        self.resolve_section(product);

        if product.price <= product.section.config().min_price {
            product.section = Section::Others;
            return
        }
    }

    fn resolve_section(&self, product: &mut Product) {
        let mut visited = HashSet::new();
        let mut current = product.section;

        for _ in 0..5 { // safety limit
            if !visited.insert(current) {
                // cycle detected
                panic!("Cycle detected {visited:?} in {:#?}", product)
            }

            let config = current.config();
            let mut next = current;

            // Force Include
            for pattern in &config.force_include {
                if RegexCache::custom_match(pattern, &product.title) {
                    return;
                }
            }

            // Title Rules
            for item in &config.move_rules {
                if let [section, pattern, ..] = item.as_slice() {
                    if RegexCache::custom_match(pattern, &product.title) {
                        next = Section::from_str(section).unwrap();
                        break;
                    }
                }
            }

            // Description Rules
            if next == current {
                if let Some(desc) = &product.description {
                    let text = format!("{} {}", product.title, desc);
                    for item in &config.move_by_description_rules {
                        if let [section, pattern, ..] = item.as_slice() {
                            if RegexCache::custom_match(pattern, &text) {
                                next = Section::from_str(section).unwrap();
                                break;
                            }
                        }
                    }
                }
            }

            if next == current {
                break;
            }

            current = next;
        }

        product.section = current;
    }

    async fn parse(&self, products: &mut Vec<Product>) {
        let start_time = Instant::now();
        for mut product in products.iter_mut() {
            self.verify_section(product);
            if let Err(error) = product.section.parser().parse(product) {
                //FIXME: remove
                if error == ParseErrorKind::NotInDataset {
                    continue;
                }
                let parse_error = ParseError { error, product: product.clone(), timestamp: Utc::now() };
                self.report.lock().await.parse.push(parse_error.clone());
                self.known_events.lock().await.parse_error(parse_error);
            }
        }
        println!("Parsed {} products in {:.2?}", products.len(), start_time.elapsed());
    }
}