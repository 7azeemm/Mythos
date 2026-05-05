use std::error::Error;
use chrono::Utc;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use serde_json::Value;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-desc-short").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span#product-availability").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "MediaVision",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products article[data-id-product]").unwrap()),
    sections: &[
        (&Section::Laptop, "https://www.mediavision.tn/24-pc-portable"),
        (&Section::GamingLaptop, "https://www.mediavision.tn/25-pc-portable-gamer"),
        (&Section::GamingLaptop, "https://www.mediavision.tn/233-pc-portable-gamer"),
        (&Section::ProLaptop, "https://www.mediavision.tn/362-pc-portable-professionnel-"),
        (&Section::ProLaptop, "https://www.mediavision.tn/363-pc-ultrabook"),
        (&Section::PC, "https://www.mediavision.tn/29-pc-de-bureau"),
        (&Section::GamingPc, "https://www.mediavision.tn/30-pc-gamer"),
        (&Section::GamingPc, "https://www.mediavision.tn/234-pc-de-bureau-gamer"),
        (&Section::PcAllInOne, "https://www.mediavision.tn/31-pc-tout-en-un"),
        (&Section::Monitor, "https://www.mediavision.tn/34-ecran"),
        (&Section::Monitor, "https://www.mediavision.tn/235-ecran"),
        (&Section::CPU, "https://www.mediavision.tn/65-processeur"),
        (&Section::CPU, "https://www.mediavision.tn/260-processeur"),
        (&Section::GPU, "https://www.mediavision.tn/66-carte-graphique"),
        (&Section::GPU, "https://www.mediavision.tn/262-carte-graphique"),
        (&Section::RAM, "https://www.mediavision.tn/57-barrette-memoire"),
        (&Section::RAM, "https://www.mediavision.tn/68-barrette-memoire"),
        (&Section::RAM, "https://www.mediavision.tn/266-barrette-memoire"),
        (&Section::MotherBoard, "https://www.mediavision.tn/67-carte-mere"),
        (&Section::MotherBoard, "https://www.mediavision.tn/261-carte-mere"),
        (&Section::SSD, "https://www.mediavision.tn/39-disque-dur-interne"),
        (&Section::SSD, "https://www.mediavision.tn/55-disque-dur"),
        (&Section::SSD, "https://www.mediavision.tn/64-disque-dur"),
        (&Section::SSD, "https://www.mediavision.tn/263-disque-dur"),
        (&Section::SSD, "https://www.mediavision.tn/264-disque-ssd"),
        (&Section::Case, "https://www.mediavision.tn/69-boitier"),
        (&Section::Case, "https://www.mediavision.tn/268-boitiers"),
        (&Section::Cooler, "https://www.mediavision.tn/62-ventilateur"),
        (&Section::Cooler, "https://www.mediavision.tn/70-refroidissement-processeur"),
        (&Section::Cooler, "https://www.mediavision.tn/265-refroidissement-processeur"),
        (&Section::Cooler, "https://www.mediavision.tn/273-ventilateur"),
        (&Section::PSU, "https://www.mediavision.tn/71-bloc-alimentation"),
        (&Section::PSU, "https://www.mediavision.tn/259-alimentation"),
    ]
};

pub struct MediaVision;

impl Site for MediaVision {
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
            true => element.select(&DESCRIPTION_SEL)
                .next()
                .ok_or("description not found")?
                .get_text()
                .split("-")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
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
            in_stock: status.contains("Disponible"),
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }
}