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

const PRODUCTS_PER_PAGE: i32 = 3;

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.tv-product-desc").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.tvproduct-cart-btn form button").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "ScoopGaming",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product p").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products article.item").unwrap()),
    sections: &[
        (&Section::GamingLaptop, "https://www.scoopgaming.com.tn/62-pc-portable-gamer"),
        (&Section::GamingPc, "https://www.scoopgaming.com.tn/56-pc-gamer"),
        (&Section::Monitor, "https://www.scoopgaming.com.tn/58-ecrans-gaming"),
        (&Section::Monitor, "https://www.scoopgaming.com.tn/61-ecrans-professionnels"),
        (&Section::CPU, "https://www.scoopgaming.com.tn/80-processeur"),
        (&Section::GPU, "https://www.scoopgaming.com.tn/39-carte-graphique"),
        (&Section::RAM, "https://www.scoopgaming.com.tn/131-memoire-pc"),
        (&Section::MotherBoard, "https://www.scoopgaming.com.tn/40-carte-mere"),
        (&Section::SSD, "https://www.scoopgaming.com.tn/15-stockage"),
        (&Section::Cooler, "https://www.scoopgaming.com.tn/49-refroidissement"),
        (&Section::Cooler, "https://www.scoopgaming.com.tn/42-ventilateur"),
        (&Section::PSU, "https://www.scoopgaming.com.tn/48-alimentation"),
        (&Section::Case, "https://www.scoopgaming.com.tn/38-boitier"),
    ]
};

pub struct ScoopGaming;

impl Site for ScoopGaming {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let in_stock = element.select(&STATUS_SEL).next().ok_or("status not found")?.attr("disabled").is_none();
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

        let text = element.replace("products", "").trim().to_string();
        let count = text.parse::<i32>()
            .map_err(|err| format!("failed to parse products count `{text}`: {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}