use crate::utils::web_client::{WebClient, WebClientType};
use crate::web_scraper::errors::{PageReport, ScrapeError, ScrapeErrorKind, SiteReport};
use crate::web_scraper::product::{Product, ProductStatus};
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::affariyet::Affariyet;
use crate::web_scraper::sites::batam::Batam;
use crate::web_scraper::sites::bestbuytunisie::BestBuyTunisie;
use crate::web_scraper::sites::carthago_informatique::CarthagoInformatique;
use crate::web_scraper::sites::cyberinfo::CyberInfo;
use crate::web_scraper::sites::expert_gaming::ExpertGaming;
use crate::web_scraper::sites::gamershop::GamerShop;
use crate::web_scraper::sites::info_tec::InfoTec;
use crate::web_scraper::sites::jmb::JMB;
use crate::web_scraper::sites::jumbo::Jumbo;
use crate::web_scraper::sites::mbm_informatique::MBMInformatique;
use crate::web_scraper::sites::media_vision::MediaVision;
use crate::web_scraper::sites::megapc::MegaPC;
use crate::web_scraper::sites::mytek::Mytek;
use crate::web_scraper::sites::sbs_informatique::SBSInformatique;
use crate::web_scraper::sites::scoop_gaming::ScoopGaming;
use crate::web_scraper::sites::sig_shop::SigShop;
use crate::web_scraper::sites::skymil_shop::SkyMilShop;
use crate::web_scraper::sites::spacenet::SpaceNet;
use crate::web_scraper::sites::tdiscount::TDiscount;
use crate::web_scraper::sites::oxtek::OXTek;
use crate::web_scraper::sites::techspace::TechSpace;
use crate::web_scraper::sites::tunewtec::TunewTec;
use crate::web_scraper::sites::tunisianet::Tunisianet;
use crate::web_scraper::utils::{extract_basics, extract_prices, validate_url, ElementRefExt};
use crate::web_scraper::sites::wiki_tn::WikiTN;
use crate::web_scraper::sites::zstore::ZStore;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::time::{Duration, Instant};
use serde_json::Value;
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::utils::file_loader::FileLoader;

pub mod tunisianet;
pub mod skymil_shop;
pub mod mytek;
pub mod gamershop;
pub mod megapc;
pub mod spacenet;
pub mod expert_gaming;
pub mod sig_shop;
pub mod carthago_informatique;
pub mod wiki_tn;
pub mod media_vision;
pub mod info_tec;
pub mod cyberinfo;
pub mod mbm_informatique;
pub mod jmb;
pub mod jumbo;
pub mod affariyet;
pub mod batam;
pub mod tunewtec;
pub mod techspace;
pub mod oxtek;
pub mod bestbuytunisie;
pub mod tdiscount;
pub mod zstore;
pub mod sbs_informatique;
pub mod scoop_gaming;

const MAX_RETRIES: i32 = 3;

pub static PAGE_CACHE: Lazy<RwLock<HashMap<String, Vec<Product>>>> = Lazy::new(|| RwLock::new(HashMap::new()));
pub static DESCRIPTION_CACHE: Lazy<RwLock<HashMap<String, ProductDescription>>> = Lazy::new(|| RwLock::new(HashMap::new()));

//clickup.tn
//qsnet.tn
//www.planete-informatique.tn
//https://xtreme-pc.tn/
//https://lofficielshop.tn/ ??
//nexuspc.shop
//leaderDeal
//techland

pub static SITES: Lazy<Vec<Box<dyn Site>>> = Lazy::new(|| vec![
    Box::new(Tunisianet), Box::new(SkyMilShop), Box::new(Mytek), Box::new(GamerShop), Box::new(MegaPC),
    Box::new(SpaceNet), Box::new(ExpertGaming), Box::new(SigShop), Box::new(CarthagoInformatique), Box::new(ScoopGaming),
    Box::new(WikiTN), Box::new(MediaVision), Box::new(InfoTec), Box::new(CyberInfo), Box::new(MBMInformatique),
    Box::new(JMB), Box::new(Jumbo), Box::new(Affariyet), Box::new(Batam), Box::new(TunewTec), Box::new(TechSpace),
    Box::new(OXTek), Box::new(BestBuyTunisie), Box::new(TDiscount), Box::new(ZStore), Box::new(SBSInformatique),
]);

pub struct SiteConfig {
    pub name: &'static str,
    pub web_client_type: WebClientType,
    pub nav_sel: Lazy<Selector>,
    pub product_sel: Lazy<Selector>,
    pub title_sel: Lazy<Selector>,
    pub image_sel: Lazy<Selector>,
    pub price_sel: Lazy<Selector>,
    pub old_price_sel: Lazy<Selector>,
    pub price_sel_2: Option<Lazy<Selector>>,
    pub status_sel: Option<Lazy<Selector>>,
    pub desc_sel: Option<Lazy<Selector>>,
    pub page_desc_sel: Option<Lazy<Selector>>,
    pub sections: &'static [(Section, &'static str)],
}

#[async_trait::async_trait]
pub trait Site: Send + Sync {
    fn config(&self) -> &SiteConfig;

    async fn scrape(&self, url: &str, section: Section) -> (SiteReport, Vec<Product>) {
        let start_time = Instant::now();
        let mut all_products = Vec::new();
        let mut pages = Vec::new();

        // Loading from cache
        if let Some(mut products) = PAGE_CACHE.read().await.get(url).cloned() {
            if !products.is_empty() {
                println!("Loaded {} products from cache", products.len());
                for mut product in products.iter_mut() {
                    product.specs = Value::default();
                    product.name = product.title.clone();
                }
                all_products.extend(products);
                return (SiteReport {
                    site: self.name().to_string(),
                    page_count: pages.len(),
                    total_products: all_products.len(),
                    pages: Vec::new()
                }, all_products)
            }
        }

        let (page_stats, products, page_count) = self.scrape_page(url, 1, section).await;
        let page_count = page_count.unwrap_or(1);
        pages.push(page_stats);
        all_products.extend(products);

        for page in 2..page_count+1 {
            let (page_stats, products, _) = self.scrape_page(url, page, section).await;
            pages.push(page_stats);
            all_products.extend(products);
        }

        let time_taken = start_time.elapsed();
        println!(
            "Successfully Scrapped {} products ({:?}, {} Pages) from {} in {:.2?}",
            all_products.len(), section, pages.len(), self.name(), time_taken
        );

        // Saving to cache
        PAGE_CACHE.write().await.insert(url.to_string(), all_products.clone());
        FileLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &*PAGE_CACHE.read().await).await.unwrap();

        (SiteReport {
            site: self.name().to_string(),
            total_products: all_products.len(),
            page_count: pages.len(),
            pages,
        }, all_products)
    }

    async fn scrape_page(&self, base_url: &str, page: i32, section: Section) -> (PageReport, Vec<Product>, Option<i32>) {
        let url = self.format_url(base_url, page);
        let mut products = Vec::new();
        let mut page_count = None;
        let mut errors = Vec::new();
        let mut retries = 0;

        while retries < MAX_RETRIES {
            retries += 1;
            let last_retry = retries == MAX_RETRIES;

            let body = match self.fetch(&url).await {
                Ok(body) => body,
                Err(err) => {
                    if last_retry {
                        let error = format!("Failed to fetch page (Attempt {}/{}): {}", retries, MAX_RETRIES, err);
                        errors.push(create_scrape_error(ScrapeErrorKind::FetchFailed(error), section, self.name(), &url));
                    }
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            let (parsed_products, parse_errors) = self.parse(section, page, body, &mut page_count);
            if parsed_products.is_empty() {
                errors.push(create_scrape_error(ScrapeErrorKind::ZeroProducts, section, self.name(), &url));
            }
            
            if (errors.is_empty() && parse_errors.is_empty()) || last_retry {
                errors.extend(parse_errors.into_iter()
                    .map(|e| create_scrape_error(ScrapeErrorKind::ParseFailed(e), section, self.name(), &url))
                    .collect::<Vec<ScrapeError>>());

                for mut product in parsed_products {
                    if let Err(err) = self.try_fetch_description(&mut product).await && last_retry {
                        errors.push(create_scrape_error(ScrapeErrorKind::ParseFailed(err), section, self.name(), &url))
                    }
                    products.push(product)
                }

               break;
            }

            errors.clear();
            sleep(Duration::from_secs(1)).await;
        }

        (PageReport {
            url,
            products: products.len(),
            errors,
        }, products, page_count)
    }

    fn parse(&self, section: Section, page: i32, body: String, page_count: &mut Option<i32>) -> (Vec<Product>, Vec<String>) {
        let doc = Html::parse_document(&body);
        let mut errors = Vec::new();

        if page == 1 {
            match self.parse_page_count(&doc) {
                Ok(count) => { let _ = page_count.insert(count); },
                Err(err) => errors.push(format!("Failed to parse page count: {err}")),
            }
        }

        let (products, errs) = self.parse_products(section, doc);
        errors.extend(errs);

        (products, errors)
    }

    fn parse_products(&self, section: Section, doc: Html) -> (Vec<Product>, Vec<String>) {
        let mut products = Vec::new();
        let mut errors = Vec::new();
        for product in doc.select(&self.config().product_sel) {
            match self.parse_product(section, product) {
                Ok(product) => products.push(product),
                Err(err) => errors.push(format!("Failed to parse product: {err}"))
            }
        }
        (products, errors)
    }

    fn parse_product(&self, section: Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let config = self.config();
        let (title, url, image) = self.parse_basics(element)?;
        let (price, old_price) = extract_prices(element, &config.price_sel, &config.old_price_sel, &config.price_sel_2)?;
        let status = self.parse_status(element)?;
        let description = match (section.config().requires_description, &config.desc_sel) {
            (true, Some(sel)) => {
                let desc = element.select_text(sel, "description")?;
                match desc.ends_with("..") {
                    true => None,
                    false => Some(desc)
                }
            },
            _ => None
        };

        Ok(Product::new(
            self.name(), url, title, section, description,
            image, status, price, old_price
        )?)
    }

    fn parse_basics(&self, element: ElementRef) -> Result<(String, String, String), String> {
        let config = self.config();
        let (title, url, image) = extract_basics(element, &config.title_sel, &config.image_sel)?;
        validate_url(&url)?;
        validate_url(&image)?;
        Ok((title, url, image))
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, String> {
        match &self.config().status_sel {
            Some(sel) => Ok(ProductStatus::from_str(&element.select_text(sel, "status")?)?),
            None => Ok(ProductStatus::InStock)
        }
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let elements = doc.select(&self.config().nav_sel).collect::<Vec<ElementRef>>();
        if elements.is_empty() || elements.len() == 1 {
            return Ok(1);
        }

        let last_page = elements.get(elements.len() - 2).ok_or("last page button not found")?;
        let button_text = last_page.get_text();
        Ok(button_text.parse::<i32>().map_err(|err| format!("button text: `{button_text}` ({err})"))?)
    }
    
    async fn try_fetch_description(&self, product: &mut Product) -> Result<(), String> {
        if product.section.config().requires_description && product.description.is_none() {
            match self.fetch_description(&product.url).await {
                Err(err) => return Err(err),
                Ok(desc) => product.description = Some(desc)
            }
        }
        Ok(())
    }

    async fn fetch_description(&self, url: &str) -> Result<String, String> {
        let cached_item = DESCRIPTION_CACHE.read().await.get(url).cloned();
        if let Some(cached) = cached_item {
            let duration = Utc::now().signed_duration_since(cached.timestamp);
            if duration > chrono::Duration::days(30) {
                DESCRIPTION_CACHE.write().await.remove(url);
            } else {
                return Ok(cached.description.clone());
            }
        }

        let Some(sel) = &self.config().page_desc_sel else {
            return Err("Page Description Selector not found".to_string())
        };

        let mut retries = 0;
        const MAX_DESC_RETRIES: i32 = 3;

        loop {
            retries += 1;
            let last_retry = retries == MAX_DESC_RETRIES;

            let page_content = match self.fetch(url).await {
                Ok(content) => content,
                Err(err) if last_retry => return Err(format!("Failed to fetch product description after {} attempts: {}", MAX_DESC_RETRIES, err)),
                Err(_) => {
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            let desc = if let Some(elem) = Html::parse_document(&page_content).select(&sel).next() {
                elem.get_text()
            } else {
                if last_retry {
                    return Err("description not found".to_string());
                }
                sleep(Duration::from_secs(1)).await;
                continue;
            };

            DESCRIPTION_CACHE.write().await.insert(url.to_string(), ProductDescription {
                description: desc.clone(),
                timestamp: Utc::now()
            });

            return Ok(desc);
        }
    }

    async fn fetch(&self, url: &str) -> Result<String, String> {
        WebClient::fetch(&url, &self.config().web_client_type)
            .await.map_err(|e| e.to_string())
    }

    fn name(&self) -> &'static str {
        self.config().name
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?page={page}")
    }
}

fn create_scrape_error(error: ScrapeErrorKind, section: Section, site: &str, url: &str) -> ScrapeError {
    ScrapeError {
        error,
        section,
        site: site.to_string(),
        url: url.to_string(),
        timestamp: Utc::now()
    }
}

pub fn get_site_from_str(site: &str) -> Option<&Box<dyn Site>> {
    SITES.iter().filter(|s| s.name() == site).next()
}

//TODO: filter old descriptions on load and save the file (to avoid growing in size)
#[derive(Clone, Deserialize, Serialize)]
pub struct ProductDescription {
    pub description: String,
    pub timestamp: DateTime<Utc>
}