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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.product-name a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.thumbnail-wrapper img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.short-description ul li").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-label span.out-of-stock").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "TunewTec",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products section.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable/"),
        (&Section::Laptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable-professionnel/"),
        (&Section::Laptop, "https://tunewtec.com/c/ordinateur-portable/macbook-macbook-pro/"),
        (&Section::GamingLaptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable-gamer/"),
        (&Section::GamingLaptop, "https://tunewtec.com/c/gaming/pc-portable-gamer-gaming/"),
        (&Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/pc-de-bureau/"),
        (&Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/mini-pc/"),
        (&Section::PC, "http://tunewtec.com/c/ordinateur-de-bureau/imac/"),
        (&Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/station-de-travail/"),
        (&Section::PcAllInOne, "https://tunewtec.com/c/ordinateur-de-bureau/pc-all-in-one/"),
        (&Section::GamingPc, "https://tunewtec.com/c/gaming/pc-gaming/"),
        (&Section::GamingPc, "https://tunewtec.com/c/ordinateur-de-bureau/pc-gamer/"),
        (&Section::Monitor, "https://tunewtec.com/c/gaming/ecran-gaming/"),
        (&Section::Monitor, "https://tunewtec.com/c/ordinateur-de-bureau/ecran/"),
        (&Section::CPU, "https://tunewtec.com/c/composant-pc-de-bureau/processeur/"),
        (&Section::GPU, "https://tunewtec.com/c/composant-pc-de-bureau/cartes-graphiques/"),
        (&Section::RAM, "https://tunewtec.com/c/composant-pc-de-bureau/barrettes-memoire-composant-pc-de-bureau/"),
        (&Section::RAM, "https://tunewtec.com/c/composant-pc-portable/barrettes-memoire/"),
        (&Section::RAM, "https://tunewtec.com/c/gaming/composant-pc-gamer/memoire-gamer/"),
        (&Section::MotherBoard, "https://tunewtec.com/c/composant-pc-de-bureau/carte-mere/"),
        (&Section::MotherBoard, "https://tunewtec.com/c/gaming/composant-pc-gamer/carte-mere-gamer/"),
        (&Section::SSD, "https://tunewtec.com/c/composant-pc-de-bureau/disque-ssd-composant-pc-de-bureau/"),
        (&Section::SSD, "https://tunewtec.com/c/composant-pc-portable/disque-hdd-composant-pc-portable/"),
        (&Section::SSD, "https://tunewtec.com/c/composant-pc-portable/disque-ssd-composant-pc-portable/"),
        (&Section::HDD, "https://tunewtec.com/c/composant-pc-de-bureau/disque-hdd-composant-pc-de-bureau/"),
        (&Section::Case, "https://tunewtec.com/c/composant-pc-de-bureau/boitier-pc/"),
        (&Section::Case, "https://tunewtec.com/c/gaming/composant-pc-gamer/boitier-gamer/"),
        (&Section::PSU, "https://tunewtec.com/c/composant-pc-de-bureau/alimentation-pc/"),
        (&Section::PSU, "https://tunewtec.com/c/gaming/composant-pc-gamer/alimentation-gamer/"),
        (&Section::Cooler, "https://tunewtec.com/c/composant-pc-de-bureau/refroidissement-pc/"),
        (&Section::Cooler, "https://tunewtec.com/c/gaming/composant-pc-gamer/ventilateur-gamer/"),
    ]
};

pub struct TunewTec;

impl Site for TunewTec {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();

        let (price, regular_price) = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => {
                let price = element.select(&PRICE_SEL_2).next().ok_or("price not found")?.get_text();
                (parse_price(&price)?, Some(parse_price(&p.get_text())?))
            },
            None => (parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?, None),
        };

        let url = title
            .value()
            .attr("href")
            .ok_or("url not found")?
            .to_string();

        let title = title.get_text();

        let description = match section.requires_description() {
            false => vec![],
            true => element.select(&DESCRIPTION_SEL)
                .map(|s| s.get_text())
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
            in_stock: status == "EN STOCK",
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }

    fn parse_page_count(&self, _doc: &Html) -> Result<i32, Box<dyn Error>> {
        Ok(1)
    }

    fn format_url(&self, url: &str, _page: i32) -> String {
        format!("{url}?per_page=400")
    }
}