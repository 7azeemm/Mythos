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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h3.wd-entities-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-image-link img[src]").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "InfoTec",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.wd-product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://infotec.tn/fr/c/ordinateurs-portables/"),
        (&Section::GamingLaptop, "https://infotec.tn/fr/c/pc-gamer/"),
        (&Section::GamingLaptop, "https://infotec.tn/fr/c/pc-portable-gamer/"),
        (&Section::PC, "https://infotec.tn/fr/c/pc-de-bureau/"),
        (&Section::GamingPc, "https://infotec.tn/fr/c/ordinateur-gamer/"),
        (&Section::GamingPc, "https://infotec.tn/fr/c/ordinateur-gamer-gaming-pc/"),
        (&Section::PcAllInOne, "https://infotec.tn/fr/c/pc-tout-en-un/"),
        (&Section::Monitor, "https://infotec.tn/fr/c/ecran/"),
        (&Section::Monitor, "https://infotec.tn/fr/c/ecran-gamer/"),
        (&Section::CPU, "https://infotec.tn/fr/c/processeur/"),
        (&Section::CPU, "https://infotec.tn/fr/c/processeur-composant-pc-gamer/"),
        (&Section::GPU, "https://infotec.tn/fr/c/carte-graphique/"),
        (&Section::GPU, "https://infotec.tn/fr/c/carte-graphique-composant-pc-gamer/"),
        (&Section::RAM, "https://infotec.tn/fr/c/barrettes-memoire/"),
        (&Section::RAM, "https://infotec.tn/fr/c/barrette-memoire-gamer/"),
        (&Section::MotherBoard, "https://infotec.tn/fr/c/carte-mere/"),
        (&Section::MotherBoard, "https://infotec.tn/fr/c/carte-mere-composant-pc-gamer/"),
        (&Section::SSD, "https://infotec.tn/fr/c/disque-dur-interne/"),
        (&Section::SSD, "https://infotec.tn/fr/c/disque-dur/"),
        (&Section::SSD, "https://infotec.tn/fr/c/disque-dur-ssd/"),
        (&Section::Case, "https://infotec.tn/fr/c/boitier/"),
        (&Section::Case, "https://infotec.tn/fr/c/boitier-pc-gamer/"),
        (&Section::PSU, "https://infotec.tn/fr/c/bloc-dalimentation/"),
        (&Section::PSU, "https://infotec.tn/fr/c/alimentation-pc-gamer/"),
        (&Section::Cooler, "https://infotec.tn/fr/c/refroidisseur-processeur-gamer/"),
        (&Section::Cooler, "https://infotec.tn/fr/c/ventilateur-gamer/"),
    ]
};

pub struct InfoTec;

impl Site for InfoTec {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let in_stock = true;

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
            .ok_or("product url not found")?
            .to_string();

        let title = title.get_text();

        let description = match section.requires_description() {
            false => vec![],
            true => todo!(),
        };

        let image = image
            .value()
            .attr("src")
            .ok_or("image url not found")?
            .to_string();

        println!("{url}, {title}, {in_stock}, {image}, {price}, {regular_price:?}, {description:?}");

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