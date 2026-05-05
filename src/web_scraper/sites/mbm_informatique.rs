use std::collections::HashMap;
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
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.tvproduct-cart-btn form button span").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "MBMInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product p").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products article.item").unwrap()),
    sections: &[
        (&Section::Laptop, "https://mbm-tn.com/145-pc-portable"),
        (&Section::Laptop, "https://mbm-tn.com/257-macbook"),
        (&Section::GamingLaptop, "https://mbm-tn.com/256-pc-portable-gamer"),
        (&Section::PcAllInOne, "https://mbm-tn.com/232-pc-tout-en-un-all-in-one"),
        (&Section::PC, "https://mbm-tn.com/76-pc-de-bureau"),
        (&Section::PC, "https://mbm-tn.com/111-mac"),
        (&Section::GamingPc, "https://mbm-tn.com/325-pc-de-bureau-gamer"),
        (&Section::Monitor, "https://mbm-tn.com/56-ecran"),
        (&Section::Monitor, "https://mbm-tn.com/291-ecran-gamer"),
        (&Section::CPU, "https://mbm-tn.com/124-processeur"),
        (&Section::GPU, "https://mbm-tn.com/86-cartes-graphique"),
        (&Section::RAM, "https://mbm-tn.com/85-barrettes-memoire-dimm"),
        (&Section::RAM, "https://mbm-tn.com/110-barrettes-memoire-so-dimm"),
        (&Section::MotherBoard, "https://mbm-tn.com/109-cartes-meres"),
        (&Section::MotherBoard, "https://mbm-tn.com/87-cartes-meres"),
        (&Section::SSD, "https://mbm-tn.com/174-disque-ssd"),
        (&Section::SSD, "https://mbm-tn.com/176-boitier-disque-dur"),
        (&Section::SSD, "https://mbm-tn.com/334-disque-dur-interne"),
        (&Section::SSD, "https://mbm-tn.com/66-disques-dur-internes"),
        (&Section::SSD, "https://mbm-tn.com/59-disque-dur-interne"),
        (&Section::Case, "https://mbm-tn.com/335-boitier"),
        (&Section::Cooler, "https://mbm-tn.com/71-ventilateurs"),
        (&Section::Cooler, "https://mbm-tn.com/155-ventilateurs"),
        (&Section::Cooler, "https://mbm-tn.com/143-refroidisseur"),
        (&Section::PSU, "https://mbm-tn.com/108-blocs-alimentation-"),
    ]
};

pub struct MBMInformatique;

#[async_trait::async_trait]
impl Site for MBMInformatique {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.value().attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();

        let description = match section.requires_description() {
            false => vec![],
            true => vec![
                element.select(&DESCRIPTION_SEL)
                    .next().ok_or("description not found")?
                    .get_text().trim().to_string()
            ]
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
            in_stock: status.contains("Add To Cart"),
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

        let text = element.replace("products", "").trim().to_string();
        let count = text.parse::<i32>()
            .map_err(|err| format!("failed to parse products count ({text}): {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}