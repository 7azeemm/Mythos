use std::error::Error;
use chrono::Utc;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use serde_json::Value;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::scheduler::print;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.product-name a").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.product-name a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.thumbnail-wrapper a figure img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.short-description").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "ExpertGaming",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav ul.page-numbers li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products section.product").unwrap()),
    sections: &[
        (&Section::GamingPc, "https://www.expert-gaming.tn/pc-gaming-bureautique/"),
        (&Section::GamingSetup, "https://www.expert-gaming.tn/full-setup-gaming/"),
        (&Section::GamingLaptop, "https://www.expert-gaming.tn/pc-portable-gaming/"),
        (&Section::Monitor, "https://www.expert-gaming.tn/ecran-gaming/"),
        (&Section::Monitor, "https://www.expert-gaming.tn/ecrans-professionnelles/"),
        (&Section::CPU, "https://www.expert-gaming.tn/processeurs-intel/"),
        (&Section::CPU, "https://www.expert-gaming.tn/processeurs-amd/"),
        (&Section::GPU, "https://www.expert-gaming.tn/carte-graphique-nvidia/"),
        (&Section::GPU, "https://www.expert-gaming.tn/carte-graphique-amd/"),
        (&Section::RAM, "https://www.expert-gaming.tn/memoire-vive/"),
        (&Section::MotherBoard, "https://www.expert-gaming.tn/carte-mere-intel/"),
        (&Section::MotherBoard, "https://www.expert-gaming.tn/carte-mere-amd/"),
        (&Section::SSD, "https://www.expert-gaming.tn/stockage/"),
        (&Section::HDD, "https://www.expert-gaming.tn/disque-interne-externe/"),
        (&Section::PSU, "https://www.expert-gaming.tn/alimentation/"),
        (&Section::Case, "https://www.expert-gaming.tn/boitier/"),
        (&Section::Cooler, "https://www.expert-gaming.tn/refroidissement-a-eau-watercooling/"),
        (&Section::Cooler, "https://www.expert-gaming.tn/refroidissement-a-air-aircooling/"),
        (&Section::Cooler, "https://www.expert-gaming.tn/ventilateur-boitier/"),
    ],
};

pub struct ExpertGaming;

impl Site for ExpertGaming {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.select(&URL_SEL).next().ok_or("url not found")?;
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?.get_text();
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let in_stock = true;

        let (price, regular_price) = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => {
                let price = element.select(&PRICE_SEL_2).next().ok_or("price not found")?.get_text();
                (parse_price(&price)?, Some(parse_price(&p.get_text())?))
            },
            None => (parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?, None),
        };

        let description = match section.requires_description() {
            false => vec![],
            true => element.select(&DESCRIPTION_SEL)
                .next()
                .ok_or("description not found")?
                .get_text()
                .split(".")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        };

        let url = url
            .value()
            .attr("href")
            .ok_or("product url not found")?
            .to_string();

        let image = image
            .value()
            .attr("data-src")
            .ok_or("image url not found")?
            .to_string();

        print(&format!("{url}, {title}, {in_stock}, {image}, {price}, {regular_price:?}, {description:?}"));

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

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}")
    }
}