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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product_name a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.decriptions-short p").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("button.add-to-cart i").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "Jumbo",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    sections: &[
        (&Section::Laptop, "https://jumbo.tn/57-pc-portable"),
        (&Section::Laptop, "https://jumbo.tn/541-macbook"),
        (&Section::ProLaptop, "https://jumbo.tn/574-pc-portable-pro"),
        (&Section::GamingLaptop, "https://jumbo.tn/140-pc-portable-gamer"),
        (&Section::GamingLaptop, "https://jumbo.tn/614-pc-portable-gamer"),
        (&Section::PC, "https://jumbo.tn/61-pc-de-bureau"),
        (&Section::PcAllInOne, "https://jumbo.tn/548-pc-tout-en-un"),
        (&Section::GamingPc, "https://jumbo.tn/141-pc-de-bureau-gamer"),
        (&Section::GamingPc, "https://jumbo.tn/615-ordinateur-de-bureau-gamer"),
        (&Section::Monitor, "https://jumbo.tn/654-ecran-pro-lfd"),
        (&Section::Monitor, "https://jumbo.tn/544-ecran"),
        (&Section::Monitor, "https://jumbo.tn/142-ecran-pc-gamer"),
        (&Section::CPU, "https://jumbo.tn/637-processeur"),
        (&Section::GPU, "https://jumbo.tn/638-carte-graphique"),
        (&Section::RAM, "https://jumbo.tn/640-barrette-memoire"),
        (&Section::MotherBoard, "https://jumbo.tn/639-carte-mere"),
        (&Section::SSD, "https://jumbo.tn/144-disque-dur-interne"),
        (&Section::SSD, "https://jumbo.tn/556-disque-dur-ssd"),
        (&Section::PSU, "https://jumbo.tn/643-bloc-d-alimentation"),
        (&Section::Case, "https://jumbo.tn/641-boitier"),
        (&Section::Case, "https://jumbo.tn/485-boitier-pc-gamer"),
        (&Section::Cooler, "https://jumbo.tn/567-refroidisseur"),
        (&Section::Cooler, "https://jumbo.tn/642-ventilateur"),
        (&Section::Cooler, "https://jumbo.tn/581-refroidisseur"),
    ]
};

pub struct Jumbo;

impl Site for Jumbo {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();

        let in_stock = element.select(&STATUS_SEL).next()
            .ok_or("status not found")?
            .attr("fa-ban")
            .map(|_| false)
            .unwrap_or(true);

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        let description = match section.requires_description() {
            false => vec![],
            true => element.select(&DESCRIPTION_SEL)
                .next()
                .ok_or("description not found")?
                .get_text()
                .split("-")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
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
            in_stock,
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }
}