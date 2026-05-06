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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-meta a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("img.hover-gallery-image").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "ZStore",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-portable/"),
        (&Section::GamingLaptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-gaming/"),
        (&Section::GamingLaptop, "https://zstore.com.tn/categorie-produit/gaming/laptop-gaming/"),
        (&Section::ProLaptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-portable-pro/"),
        (&Section::PC, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/pc-de-bureau/"),
        (&Section::PC, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/mini-pc/"),
        (&Section::GamingPc, "https://zstore.com.tn/categorie-produit/gaming/ordinateurs-de-bureau-gaming/"),
        (&Section::GamingSetup, "https://zstore.com.tn/categorie-produit/gaming/full-setup-gamer/"),
        (&Section::PcAllInOne, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/pc-tout-en-un/"),
        (&Section::Monitor, "https://zstore.com.tn/categorie-produit/gaming/ecran-gaming/"),
        (&Section::CPU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/processeur/"),
        (&Section::CPU, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/processeur-gaming/"),
        (&Section::GPU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/carte-graphique/"),
        (&Section::GPU, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/carte-graphique-gaming/"),
        (&Section::RAM, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/barrette-memoire/"),
        (&Section::MotherBoard, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/cartes-meres/"),
        (&Section::MotherBoard, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/cartes-meres-gaming/"),
        (&Section::SSD, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/disque-dur-interne/"),
        (&Section::PSU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/bloc-dalimentation/"),
        (&Section::Cooler, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/ventilateur-et-refroidisseur/"),
        (&Section::Cooler, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/refroidissement/"),
        (&Section::Case, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/boitier/"),
        (&Section::Case, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/boitiers-gaming/"),
    ]
};

pub struct ZStore;

impl Site for ZStore {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let url = title.value().attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();
        let in_stock = true;

        let (price, regular_price) = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => {
                let price = element.select(&PRICE_SEL_2).next().ok_or("price not found")?.get_text();
                (parse_price(&price)?, Some(parse_price(&p.get_text())?))
            },
            None => (parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?, None),
        };

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
            in_stock,
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
        url.to_string()
    }
}