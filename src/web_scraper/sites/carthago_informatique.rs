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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title a[href]").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.xts-product-image img").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.berocket_better_labels span b[style]").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "CarthagoInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.xts-col").unwrap()),
    sections: &[
        (&Section::PC, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-de-bureau/"),
        (&Section::GamingPc, "https://carthagoinformatique.tn/categorie-produit/gaming/pc-gamer/"),
        (&Section::PcAllInOne, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-tout-en-un/"),
        (&Section::Monitor, "https://carthagoinformatique.tn/categorie-produit/informatique/accessoires-ordinateur/ecran/"),
        (&Section::Laptop, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-portable/"),
        (&Section::GamingLaptop, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-portable-gamer/"),
        (&Section::CPU, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/processeur/"),
        (&Section::GPU, "https://carthagoinformatique.tn/categorie-produit/gaming/composants/carte-graphique/"),
        (&Section::MotherBoard, "https://carthagoinformatique.tn/categorie-produit/informatique/composants-pc/carte-mere-pc/"),
        (&Section::RAM, "https://carthagoinformatique.tn/categorie-produit/gaming/composants/barrette-memoire/"),
        (&Section::SSD, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/disque-dur-ssd-hdd-mvme/"),
        (&Section::SSD, "https://carthagoinformatique.tn/categorie-produit/informatique/stockage/disque-dur-interne/"),
        (&Section::Cooler, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/refroidissement/"),
        (&Section::PSU, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/bloc-dalimentation/"),
        (&Section::PSU, "https://carthagoinformatique.tn/categorie-produit/informatique/composants-pc/bloc-dalimentation-pc/"),
        (&Section::Case, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/boitier-pc-gamer/"),
    ],
};

pub struct CarthagoInformatique;

impl Site for CarthagoInformatique {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.select(&URL_SEL).next().ok_or("url not found")?;
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?.get_text();
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();

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

        let url = url
            .value()
            .attr("href")
            .ok_or("product url not found")?
            .to_string();

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

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}