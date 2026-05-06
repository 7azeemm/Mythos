use crate::utils::web_client::{WebClient, WebClientType};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::skymil_shop::SkyMilShop;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
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
use crate::web_scraper::sites::scoop::Scoop;
use crate::web_scraper::sites::scoop_gaming::ScoopGaming;
use crate::web_scraper::sites::sig_shop::SigShop;
use crate::web_scraper::sites::spacenet::SpaceNet;
use crate::web_scraper::sites::tdiscount::TDiscount;
use crate::web_scraper::sites::technopro::TechnoPro;
use crate::web_scraper::sites::techspace::TechSpace;
use crate::web_scraper::sites::tunewtec::TunewTec;
use crate::web_scraper::sites::tunisianet::Tunisianet;
use crate::web_scraper::sites::utils::ElementRefExt;
use crate::web_scraper::sites::wiki_tn::WikiTN;
use crate::web_scraper::sites::zstore::ZStore;

pub mod tunisianet;
pub mod utils;
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
pub mod technopro;
pub mod bestbuytunisie;
pub mod tdiscount;
pub mod zstore;
pub mod sbs_informatique;
pub mod scoop;
pub mod scoop_gaming;

pub static PAGE_CACHE: Lazy<RwLock<HashMap<String, HashMap<String, Product>>>> = Lazy::new(|| RwLock::new(HashMap::new()));
const MAX_RETRIES: i32 = 3;

//print(&format!("{url}, {title}, {status}, {image}, {price}, {regular_price:?}, {description:?}"));
//println!("{url}, {title}, {status}, {image}, {price}, {regular_price:?}, {description:?}");

//clickup.tn
//qsnet.tn
//www.planete-informatique.tn
//https://xtreme-pc.tn/
//https://lofficielshop.tn/ ??
//nexuspc.shop
//leaderDeal

pub static SITES: Lazy<Vec<Box<dyn Site>>> = Lazy::new(|| vec![
    // Box::new(Tunisianet),
    // Box::new(SkyMilShop),
    // Box::new(Mytek),
    // Box::new(GamerShop),
    // Box::new(MegaPC),
    // Box::new(SpaceNet),
    // Box::new(ExpertGaming),
    // Box::new(SigShop),
    // Box::new(CarthagoInformatique),
    // Box::new(WikiTN),
    // Box::new(MediaVision),
    // Box::new(InfoTec),
    // Box::new(CyberInfo),
    // Box::new(MBMInformatique),
    // Box::new(JMB),
    // Box::new(Jumbo),
    // Box::new(Affariyet),
    // Box::new(Batam),
    // Box::new(TunewTec),
    // Box::new(TechSpace),
    // Box::new(TechnoPro),
    // Box::new(BestBuyTunisie),
    // Box::new(TDiscount),
    // Box::new(ZStore),
    // Box::new(SBSInformatique),
    // Box::new(Scoop),
    // Box::new(ScoopGaming),
]);

pub struct SiteConfig {
    pub name: &'static str,
    pub web_client_type: WebClientType,
    pub nav_selector: Lazy<Selector>,
    pub product_selector: Lazy<Selector>,
    pub sections: &'static [(&'static Section, &'static str)],
}

#[async_trait::async_trait]
pub trait Site: Send + Sync {
    fn config(&self) -> &SiteConfig;
    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>>;

    async fn scrape(&self, url: &str, section: &Section, products: &mut HashMap<String, Product>) {
        let start_time = Instant::now();

        // Loading from cache
        // if let Some(cached_products) = PAGE_CACHE.read().await.get(url).cloned() {
        //     println!("Loaded {} products from cache", cached_products.len());
        //     products.extend(cached_products);
        //     return;
        // }

        let mut products_list = HashMap::default();
        if let Some(page_count) = self.scrape_page(url, 1, section, &mut products_list).await {
            for page in 2..page_count+1 {
                let _ = self.scrape_page(url, page, section, &mut products_list).await;
            }
        }

        // Saving to cache
        let count = products_list.len();
        PAGE_CACHE.write().await.insert(url.to_string(), products_list.clone());
        products.extend(products_list);

        println!(
            "Scraped in {:.2?} ({} products)",
            start_time.elapsed(),
            count
        );
    }

    async fn scrape_page(&self, base_url: &str, page: i32, section: &Section, products: &mut HashMap<String, Product>) -> Option<i32> {
        let url = self.format_url(base_url, page);
        let mut retries = 0;
        let mut page_count = None;

        while retries < MAX_RETRIES {
            let body = match self.fetch(&url).await {
                Ok(b) => b,
                Err(err) => {
                    eprintln!("Failed to fetch page {page} (Attempt {}/{MAX_RETRIES}): {err}", retries + 1);
                    retries += 1;
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };

            let parse_result = self.parse(section, page, body, &mut page_count);
            if page == 1 && let Some(count) = page_count {
                println!("Found {count} pages");
            }

            let parsed = match parse_result {
                Ok(p) => p,
                Err(err) => {
                    eprintln!("{} (Attempt {}/{MAX_RETRIES})", err, retries + 1);
                    retries += 1;
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };

            let count = parsed.len();
            for mut product in parsed {
                if let Some(existing) = products.get(&product.url) {
                    for section in &existing.sections {
                        if !product.sections.contains(section) {
                            product.sections.push(section.clone());
                        }
                    }
                }
                products.insert(product.url.clone(), product);
            }

            println!("Scraped page {page} ({count} products)");
            return page_count;
        }

        eprintln!("Giving up on page {page} after {MAX_RETRIES} attempts ({url})");
        page_count
    }

    fn parse(&self, section: &Section, page: i32, body: String, page_count: &mut Option<i32>) -> Result<Vec<Product>, String> {
        let doc = Html::parse_document(&body);
        let mut error = None;

        if page == 1 {
            match self.parse_page_count(&doc) {
                Ok(count) => { let _ = page_count.insert(count); },
                Err(err) => error = Some(err.to_string()),
            }
        }

        match error {
            Some(err) => Err(format!("Failed to parse page count: {err}")),
            None => match self.parse_products(section, doc) {
                Ok(list) => match list.len() {
                    0 => Err(format!("Found 0 products on page {page}")),
                    _ => Ok(list)
                },
                Err(err) => Err(format!("Failed to parse products in page {page}: {err}")),
            }
        }
    }

    fn parse_products(&self, section: &Section, doc: Html) -> Result<Vec<Product>, Box<dyn Error>> {
        let mut products = Vec::new();
        for product in doc.select(&self.config().product_selector) {
            match self.parse_product(section, product) {
                Ok(product) => products.push(product),
                Err(err) => eprintln!("Failed to parse product: {err}")
            }
        }
        Ok(products)
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let elements = doc.select(&self.config().nav_selector).collect::<Vec<ElementRef>>();
        if elements.is_empty() || elements.len() == 1 {
            return Ok(1);
        }

        let last_page = elements.get(elements.len() - 2).ok_or("last page button not found")?;
        let button_text = last_page.get_text();
        Ok(button_text.parse::<i32>().map_err(|err| format!("button text: `{button_text}` ({err})"))?)
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