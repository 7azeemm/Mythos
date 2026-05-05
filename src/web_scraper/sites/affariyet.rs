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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-name a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-thumbnail a.product-cover-link img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-description-short").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "Affariyet",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.product-list div[data-id-product] article").unwrap()),
    sections: &[
        (&Section::PC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/pc-de-bureau/"),
        (&Section::PC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/imac/"),
        (&Section::GamingPc, "https://www.affariyet.com/gaming/pc-gamer/ordinateur-gamer-/"),
        (&Section::PcAllInOne, "https://www.affariyet.com/informatique/ordinateur-de-bureau/tout-en-un-/"),
        (&Section::Laptop, "https://www.affariyet.com/informatique/ordinateurs-portables/"),
        (&Section::GamingLaptop, "https://www.affariyet.com/gaming/pc-gamer/pc-portable-gamer/"),
        (&Section::Monitor, "https://www.affariyet.com/informatique/ordinateur-de-bureau/ecran/"),
        (&Section::Monitor, "https://www.affariyet.com/gaming/pc-gamer/ecran-gamer/"),
        (&Section::CPU, "https://www.affariyet.com/informatique/composants-informatique-/processeur/"),
        (&Section::GPU, "https://www.affariyet.com/informatique/composants-informatique-/carte-graphique/"),
        (&Section::GPU, "https://www.affariyet.com/gaming/composants-gamer/carte-graphique-/"),
        (&Section::RAM, "https://www.affariyet.com/informatique/composants-informatique-/barrettes-memoire/"),
        (&Section::RAM, "https://www.affariyet.com/gaming/composants-gamer/barrette-memoire-gamer/"),
        (&Section::MotherBoard, "https://www.affariyet.com/informatique/composants-informatique-/carte-mere/"),
        (&Section::SSD, "https://www.affariyet.com/informatique/stockage-/disques-durs-internes/"),
        (&Section::PSU, "https://www.affariyet.com/informatique/composants-informatique-/bloc-d-alimentation/"),
        (&Section::PSU, "https://www.affariyet.com/gaming/composants-gamer/alimentation-pc-gamer/"),
        (&Section::Cooler, "https://www.affariyet.com/gaming/composants-gamer/ventilateur-refroidisseur-pc/"),
        (&Section::Case, "https://www.affariyet.com/gaming/composants-gamer/boitier-gaming/"),
    ]
};

pub struct Affariyet;

impl Site for Affariyet {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();
        let in_stock = true;

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
            .attr("data-original")
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