use std::error::Error;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.wd-entities-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-image-link img[src]").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "LeaderDeal",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.wd-product").unwrap()),
    sections: &[]
};

pub struct LeaderDeal;

impl Site for LeaderDeal {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        todo!()
    }
}