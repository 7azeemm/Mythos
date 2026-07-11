use crate::utils::file_loader::FileLoader;
use crate::utils::web_client::{WebClient, WebClientType};
use crate::web_scraper::errors::{PageReport, ScrapeError, ScrapeErrorKind};
use crate::web_scraper::product::{Product, ProductStatus};
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::affariyet::Affariyet;
use crate::web_scraper::sites::agora::Agora;
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
use crate::web_scraper::sites::megapc::MegaPC;
use crate::web_scraper::sites::mytek::Mytek;
use crate::web_scraper::sites::sbs_informatique::SBSInformatique;
use crate::web_scraper::sites::scoop_gaming::ScoopGaming;
use crate::web_scraper::sites::sig_shop::SigShop;
use crate::web_scraper::sites::skymil_shop::SkyMilShop;
use crate::web_scraper::sites::spacenet::SpaceNet;
use crate::web_scraper::sites::tdiscount::TDiscount;
use crate::web_scraper::sites::techspace::TechSpace;
use crate::web_scraper::sites::tunewtec::TunewTec;
use crate::web_scraper::sites::tunisianet::Tunisianet;
use crate::web_scraper::sites::wiki_tn::WikiTN;
use crate::web_scraper::utils::{extract_basics, extract_prices, validate_url};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::utils::scraper_ext::ElementRefExt;
use crate::web_scraper::sites::gamezone::GameZone;

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
pub mod bestbuytunisie;
pub mod tdiscount;
pub mod sbs_informatique;
pub mod scoop_gaming;
pub mod agora;
pub mod gamezone;

const MAX_RETRIES: i32 = 3;
const DESCRIPTION_DURATION: chrono::Duration = chrono::Duration::days(90);

pub static PAGE_CACHE: Lazy<RwLock<HashMap<String, Vec<Product>>>> = Lazy::new(|| RwLock::new(HashMap::new()));
pub static DESCRIPTION_CACHE: Lazy<RwLock<HashMap<String, ProductDescription>>> = Lazy::new(|| RwLock::new(HashMap::new()));

//https://www.alltecdist.com/
//microzone
//tawem
//https://www.lazari.tn/
//https://www.sws-informatique.com/
//Box::new(MediaVision) product titles are trimmed

pub static SITES: Lazy<Vec<Box<dyn Site>>> = Lazy::new(|| vec![
    Box::new(Tunisianet), Box::new(SkyMilShop), Box::new(Mytek), Box::new(GamerShop), Box::new(MegaPC),
    Box::new(SpaceNet), Box::new(ExpertGaming), Box::new(SigShop), Box::new(CarthagoInformatique), Box::new(ScoopGaming),
    Box::new(WikiTN), Box::new(InfoTec), Box::new(CyberInfo), Box::new(MBMInformatique),
    Box::new(JMB), Box::new(Jumbo), Box::new(Affariyet), Box::new(Batam), Box::new(TunewTec), Box::new(TechSpace),
    Box::new(BestBuyTunisie), Box::new(TDiscount), Box::new(SBSInformatique), Box::new(Agora), Box::new(GameZone)
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
    pub empty_page_sel: Option<Lazy<Selector>>,
    pub sections: &'static [(Section, &'static str)],
}

#[async_trait::async_trait]
pub trait Site: Send + Sync {
    fn config(&self) -> &SiteConfig;

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
                Err(err) if last_retry => {
                    let error = format!("Failed to fetch page (Attempt {}/{}): {}", retries, MAX_RETRIES, err);
                    errors.push(ScrapeErrorKind::FetchFailed(error));
                    break;
                },
                Err(_) => {
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }
            };

            let (parsed_products, parse_errors, page_empty) = self.parse(section, page, body, &mut page_count);

            products.extend(parsed_products);
            if !page_empty && products.is_empty() {
                errors.push(ScrapeErrorKind::FetchFailed("Zero Products".to_string()));
            }

            if errors.is_empty() || last_retry {
                errors.extend(parse_errors);
                break;
            }

            errors.clear();
            sleep(Duration::from_millis(500)).await;
        }

        // Fetch Descriptions
        for mut product in products.iter_mut() {
            if product.section.requires_desc() && product.description.is_none() {
                match self.fetch_description(&product.url).await {
                    Ok(desc) => product.description = Some(desc),
                    Err(err) => errors.push(ScrapeErrorKind::DescriptionFetchFailed {
                        url: product.url.clone(),
                        title: product.title.clone(),
                        error: err
                    })
                }
            }
        }

        // Saving to cache
        PAGE_CACHE.write().await.insert(url.clone(), products.clone());
        FileLoader::save_to_file::<HashMap<String, Vec<Product>>>("pages_cache.json", &*PAGE_CACHE.read().await).await.unwrap();

        (PageReport {
            products: products.len(),
            errors: errors.into_iter().map(|e| ScrapeError::new(e, section, self.name(), &url)).collect(),
            url
        }, products, page_count)
    }

    fn parse(&self, section: Section, page: i32, body: String, page_count: &mut Option<i32>) -> (Vec<Product>, Vec<ScrapeErrorKind>, bool) {
        let mut errors = Vec::new();
        let doc = Html::parse_document(&body);

        if self.check_if_page_empty(&doc) {
            return (vec![], errors, true)
        }

        if page == 1 {
            match self.parse_page_count(&doc) {
                Ok(count) => *page_count = Some(count),
                Err(err) => errors.push(ScrapeErrorKind::PageCountParseFailed(err.to_string()))
            }
        }

        let (products, parse_errors) = self.parse_products(section, doc);
        errors.extend(parse_errors);

        (products, errors, false)
    }

    fn parse_products(&self, section: Section, doc: Html) -> (Vec<Product>, Vec<ScrapeErrorKind>) {
        let mut products = Vec::new();
        let mut errors = Vec::new();
        for (i, product) in doc.select(&self.config().product_sel).enumerate() {
            match self.parse_product(section, product) {
                Ok(product) => products.push(product),
                Err(err) => errors.push(ScrapeErrorKind::ParseFailed {
                    position: i,
                    error: err.to_string()
                })
            }
        }
        (products, errors)
    }

    fn parse_product(&self, section: Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let config = self.config();
        let (title, url, image) = self.parse_basics(element)?;
        let (price, old_price) = extract_prices(element, &config.price_sel, &config.old_price_sel, &config.price_sel_2)?;
        let status = self.parse_status(element)?;
        let description = match (section.requires_desc(), &config.desc_sel) {
            (true, Some(sel)) => match element.select_text(sel, "description") {
                Ok(desc) => match desc.ends_with("..") {
                    true => None,
                    false => Some(desc)
                }
                Err(_) => None
            }
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

    fn check_if_page_empty(&self, doc: &Html) -> bool {
        if let Some(sel) = &self.config().empty_page_sel {
            if doc.select(sel).next().is_some() {
                return true
            }
        }
        false
    }
    
    async fn fetch_description(&self, url: &str) -> Result<String, String> {
        let cached_item = DESCRIPTION_CACHE.read().await.get(url).cloned();
        if let Some(cached) = cached_item {
            let duration = Utc::now().signed_duration_since(cached.timestamp);
            if duration > DESCRIPTION_DURATION {
                DESCRIPTION_CACHE.write().await.remove(url);
            } else {
                return Ok(cached.description.clone());
            }
        }

        let Some(sel) = &self.config().page_desc_sel else {
            return Err("Page Description Selector not found".to_string())
        };

        let mut retries = 0;

        loop {
            retries += 1;
            let last_retry = retries == MAX_RETRIES;

            let page_content = match self.fetch(url).await {
                Ok(content) => content,
                Err(err) if last_retry => return Err(format!("Failed to fetch product description after {MAX_RETRIES} attempts: {err}")),
                Err(_) => {
                    sleep(Duration::from_millis(500)).await;
                    continue;
                }
            };

            let desc = if let Some(elem) = Html::parse_document(&page_content).select(&sel).next() {
                elem.get_text()
            } else if last_retry {
                return Err("description not found".to_string());
            } else {
                sleep(Duration::from_millis(500)).await;
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
        WebClient::fetch(&url, &self.config().web_client_type).await.map_err(|e| e.to_string())
    }

    fn name(&self) -> &'static str {
        self.config().name
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?page={page}")
    }
}

//TODO: filter old descriptions on load and save the file (to avoid growing in size)
#[derive(Clone, Deserialize, Serialize)]
pub struct ProductDescription {
    pub description: String,
    pub timestamp: DateTime<Utc>
}