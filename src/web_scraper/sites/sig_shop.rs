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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.woocommerce-loop-product__link").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-short-description p").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "SigShop",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav ul.page-numbers li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://sig-shop.tn/categorie-produit/pc-portable/"),
        (&Section::GamingLaptop, "https://sig-shop.tn/categorie-produit/gaming-2/pc-portable-gaming-2/"),
        (&Section::PC, "https://sig-shop.tn/categorie-produit/pc-bureau/"),
        (&Section::GamingPc, "https://sig-shop.tn/categorie-produit/gaming-2/pc-bureau-gaming-2/"),
        (&Section::PcAllInOne, "https://sig-shop.tn/categorie-produit/pc-bureau/all-in-one/"),
        (&Section::Monitor, "https://sig-shop.tn/categorie-produit/accessoires/ecran/"),
        (&Section::CPU, "https://sig-shop.tn/categorie-produit/accessoires/processeur/"),
        (&Section::GPU, "https://sig-shop.tn/categorie-produit/accessoires/carte-graphique/"),
        (&Section::RAM, "https://sig-shop.tn/categorie-produit/accessoires/barrette-memoire/"),
        (&Section::MotherBoard, "https://sig-shop.tn/categorie-produit/accessoires/carte-mere-accessoires/"),
        (&Section::Case, "https://sig-shop.tn/categorie-produit/accessoires/boitier/"),
        (&Section::SSD, "https://sig-shop.tn/categorie-produit/accessoires/stockage/disque-dur-interne/"),
    ]
};

pub struct SigShop;

impl Site for SigShop {
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

        let url = url
            .value()
            .attr("href")
            .ok_or("product url not found")?
            .to_string();

        let description = match section.requires_description() {
            false => vec![],
            true => element.select(&DESCRIPTION_SEL)
                .next()
                .map(|e| e.get_text()
                    .split("-")
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>())
                .unwrap_or_else(|| {
                    eprintln!("description not found of product {url}");
                    vec![]
                }),
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

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}