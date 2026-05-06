use std::error::Error;
use chrono::Utc;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

const PRODUCTS_PER_PAGE: i32 = 12;

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.tv-product-desc").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.tvproduct-cart-btn form button").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "SBSInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product h2").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products article.item").unwrap()),
    sections: &[
        (&Section::GamingPc, "https://www.sbsinformatique.com/pc-gamer-tunisie"),
        (&Section::GamingPc, "https://www.sbsinformatique.com/stations-pro-tunisie"),
        (&Section::PC, "https://www.sbsinformatique.com/pcs-de-bureau-tunisie"),
        (&Section::GamingLaptop, "https://www.sbsinformatique.com/pc-portable-tunisie"),
        (&Section::Monitor, "https://www.sbsinformatique.com/moniteurs-tunisie"),
        (&Section::CPU, "https://www.sbsinformatique.com/processeur-tunisie"),
        (&Section::GPU, "https://www.sbsinformatique.com/cartes-graphiques-tunisie"),
        (&Section::RAM, "https://www.sbsinformatique.com/barrettes-memoires-tunisie"),
        (&Section::MotherBoard, "https://www.sbsinformatique.com/carte-mere-tunisie"),
        (&Section::SSD, "https://www.sbsinformatique.com/stockage-hdd-ssd-tunisie"),
        (&Section::Case, "https://www.sbsinformatique.com/boitiers-pc-tunisie"),
        (&Section::PSU, "https://www.sbsinformatique.com/alimentations-tunisie"),
        (&Section::Cooler, "https://www.sbsinformatique.com/refroidissement-boitier-tunisie"),
        (&Section::Cooler, "https://www.sbsinformatique.com/refroidissement-cpu-tunisie"),
    ]
};

pub struct SBSInformatique;

impl Site for SBSInformatique {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.value().attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();
        let in_stock = element.select(&STATUS_SEL).next().ok_or("status not found")?.attr("disabled").is_none();

        let description = match section.requires_description() {
            false => vec![],
            true => todo!()
        };

        let image = image
            .value()
            .attr("src")
            .ok_or("image url not found")?
            .to_string();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title,
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock,
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let element = doc.select(&self.config().nav_selector)
            .next()
            .ok_or("products count not found")?
            .get_text();

        let text = element.replace("produits", "").trim().to_string();
        let count = text.parse::<i32>()
            .map_err(|err| format!("failed to parse products count `{text}`: {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}