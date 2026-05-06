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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-availability span").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-description-short").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.product-price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

//FIXME: OXtek?
static CONFIG: SiteConfig = SiteConfig {
    name: "TechnoPro",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products article.product-miniature").unwrap()),
    sections: &[
        (&Section::Laptop, "https://www.technopro-online.com/prix-pc-portable-hp-dell-asus-lenovo-acer-Tunisie.html"),
        (&Section::Laptop, "https://www.technopro-online.com/prix-macbook-tunisie.html"),
        (&Section::GamingPc, "https://www.technopro-online.com/pc-gamer.html"),
        (&Section::PC, "https://www.technopro-online.com/pc-de-bureau.html"),
        (&Section::PC, "https://www.technopro-online.com/prix-apple-imac-tunisie.html"),
        (&Section::PcAllInOne, "https://www.technopro-online.com/prix-pc-de-bureau-tout-en-un-tunisie.html"),
        (&Section::GamingLaptop, "https://www.technopro-online.com/pc-portable-gamer.html"),
        (&Section::Monitor, "https://www.technopro-online.com/prix-ecran-ordinateur-moniteur-samsung-dell-hp-lenovo-acer-lg-tunisie.html"),
        (&Section::Monitor, "https://www.technopro-online.com/-ecran-gamer.html"),
        (&Section::CPU, "https://www.technopro-online.com/processeurs.html"),
        (&Section::GPU, "https://www.technopro-online.com/cartes-graphiques-msi-asus-macy-tunisie.html"),
        (&Section::RAM, "https://www.technopro-online.com/barrette-memoire-pour-pc-de-bureau.html"),
        (&Section::MotherBoard, "https://www.technopro-online.com/carte-mere-pour-pc-de-bureau-.html"),
        (&Section::SSD, "https://www.technopro-online.com/disques-durs-internes.html"),
        (&Section::SSD, "https://www.technopro-online.com/disque-dur-ssd-tunisie.html"),
        (&Section::PSU, "https://www.technopro-online.com/bloc-d-alimentation-.html"),
        (&Section::Case, "https://www.technopro-online.com/boitier-pc-gamer-.html"),
        (&Section::Cooler, "https://www.technopro-online.com/ventilateur-gamer-.html"),
        (&Section::Cooler, "https://www.technopro-online.com/prix-systemes-de-refroidissement-tunisie.html"),
    ]
};

pub struct TechnoPro;

impl Site for TechnoPro {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        let description = match section.requires_description() {
            true => vec![],
            false => vec![
                element.select(&DESCRIPTION_SEL)
                    .next()
                    .ok_or("description not found")?
                    .get_text()
            ],
        };

        let image = image
            .value()
            .attr("data-src")
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
}