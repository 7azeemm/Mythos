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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.woocommerce-loop-product__title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-image img[src]").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.inventory_status").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "TechSpace",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://techspace.tn/pc-portable-tunisie/"),
        (&Section::GamingPc, "https://techspace.tn/pc-gamer-tunisie/"),
        (&Section::GamingSetup, "https://techspace.tn/full-setup/"),
        (&Section::Monitor, "https://techspace.tn/ecrans-gaming/"),
        (&Section::CPU, "https://techspace.tn/processeur-intel/"),
        (&Section::CPU, "https://techspace.tn/processeur-amd/"),
        (&Section::GPU, "https://techspace.tn/carte-graphique/"),
        (&Section::RAM, "https://techspace.tn/barette-memoire/"),
        (&Section::MotherBoard, "https://techspace.tn/carte-mere-intel/"),
        (&Section::MotherBoard, "https://techspace.tn/carte-mere-amd/"),
        (&Section::SSD, "https://techspace.tn/stockage/"),
        (&Section::PSU, "https://techspace.tn/boite-dalimentation/"),
        (&Section::Cooler, "https://techspace.tn/air-cooling/"),
        (&Section::Cooler, "https://techspace.tn/water-cooling/"),
        (&Section::Cooler, "https://techspace.tn/ventilateur/"),
        (&Section::Case, "https://techspace.tn/boitier/"),
    ]
};

pub struct TechSpace;

impl Site for TechSpace {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?
            .get_text().replace("Availability:", "").trim().to_string();

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
            true => vec![],
            false => todo!(),
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
            in_stock: status == "In Stock",
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}