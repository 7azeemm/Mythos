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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.product-card__title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-card__top figure img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.decriptions-short").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-availability div[data-stock-status]").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("p.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("p.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("p.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "WikiTN",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products-grid__grid div.product-card--grid").unwrap()),
    sections: &[
        (&Section::Laptop, "https://wiki.tn/pc-portables/"),
        (&Section::GamingLaptop, "https://wiki.tn/pc-portable-gamer"),
        (&Section::PC, "https://wiki.tn/ordinateur-de-bureau"),
        (&Section::GamingPc, "https://wiki.tn/pc-gamer/"),
        (&Section::PcAllInOne, "https://wiki.tn/pc-all-in-one/"),
        (&Section::CPU, "https://wiki.tn/processeur/"),
        (&Section::GPU, "https://wiki.tn/carte-graphique"),
        (&Section::RAM, "https://wiki.tn/barrette-memoire-composants-maintenance"),
        (&Section::RAM, "https://wiki.tn/barrette-memoire-rgb-composants-gamer/"),
        (&Section::MotherBoard, "https://wiki.tn/carte-mere/"),
        (&Section::SSD, "https://wiki.tn/disque-dur-interne/"),
        (&Section::Cooler, "https://wiki.tn/ventilateurs-composants-maintenance/"),
        (&Section::Cooler, "https://wiki.tn/watercooling/"),
        (&Section::Cooler, "https://wiki.tn/ventilateurs-rgb"),
        (&Section::Monitor, "https://wiki.tn/ecran/"),
        (&Section::Case, "https://wiki.tn/boitier"),
        (&Section::PSU, "https://wiki.tn/boite-dalimentation/"),
    ]
};

pub struct WikiTN;

impl Site for WikiTN {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();

        let url = title
            .value()
            .attr("href")
            .ok_or("url not found")?
            .to_string();

        let title = title.get_text();

        let (price, regular_price) = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => {
                let price = element.select(&PRICE_SEL_2).next().ok_or("price not found")?.get_text();
                (parse_price(&price)?, Some(parse_price(&p.get_text())?))
            },
            None => (parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?, None),
        };

        let description = match section.requires_description() {
            false => vec![],
            true => todo!(),
        };

        let image = image
            .value()
            .attr("src")
            .ok_or("image url not found")?
            .to_string();

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title,
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status == "En Stock",
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?_pagination={page}")
    }
}