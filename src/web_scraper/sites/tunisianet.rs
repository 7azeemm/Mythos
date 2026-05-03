use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};
use crate::web_scraper::sites::{Site, SiteConfig};
use chrono::Utc;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use serde_json::Value;
use std::error::Error;

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[data-full-size-image-url]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse(r#"div.product-description div[itemprop="description"]"#).unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div#stock_availability").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "Tunisianet",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.item-product").unwrap()),
    sections: &[
        (&Section::PC, "https://www.tunisianet.com.tn/373-pc-de-bureau"),
        (&Section::GamingPc, "https://www.tunisianet.com.tn/682-pc-de-bureau-gamer"),
        (&Section::PcAllInOne, "https://www.tunisianet.com.tn/686-pc-tout-en-un"),
        (&Section::GamingSetup, "https://www.tunisianet.com.tn/732-full-setup-gamer"),
        (&Section::Laptop, "https://www.tunisianet.com.tn/301-pc-portable-tunisie"),
        (&Section::GamingLaptop, "https://www.tunisianet.com.tn/681-pc-portable-gamer"),
        (&Section::ProLaptop, "https://www.tunisianet.com.tn/703-pc-portable-pro"),
        (&Section::Monitor, "https://www.tunisianet.com.tn/667-ecran-pc-tunisie"),
        (&Section::Mouse, "https://www.tunisianet.com.tn/334-souris-informatique"),
        (&Section::KeyBoard, "https://www.tunisianet.com.tn/704-claviers"),
        (&Section::CPU, "https://www.tunisianet.com.tn/421-processeur"),
        (&Section::GPU, "https://www.tunisianet.com.tn/410-carte-graphique-tunisie"),
        (&Section::RAM, "https://www.tunisianet.com.tn/409-barrette-memoire"),
        (&Section::MotherBoard, "https://www.tunisianet.com.tn/420-carte-mere"),
        (&Section::HDD, "https://www.tunisianet.com.tn/408-disque-dur-interne"),
        (&Section::SSD, "https://www.tunisianet.com.tn/379-disques-ssd"),
        (&Section::Cooler, "https://www.tunisianet.com.tn/427-refroidisseur-ventilateur-boitier"),
        (&Section::Case, "https://www.tunisianet.com.tn/425-boitier"),
        (&Section::PSU, "https://www.tunisianet.com.tn/423-boite-alimentation-pc-tunisie"),
    ],
};

pub struct Tunisianet;

impl Site for Tunisianet {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.select(&URL_SEL).next().ok_or("url not found")?;
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?;
        let price = element.select(&PRICE_SEL).next().ok_or("price not found")?;

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

        let url = url
            .value()
            .attr("href")
            .ok_or("product url not found")?
            .to_string();

        let image = image
            .value()
            .attr("data-full-size-image-url")
            .or_else(|| image.value().attr("src"))
            .ok_or("image url not found")?
            .to_string();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title: title.get_text(),
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status.get_text() == "En stock",
            price: parse_price(&price.get_text())?,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }
}