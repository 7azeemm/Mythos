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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title a[href]").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.xts-product-image img").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.berocket_better_labels span b[style]").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price span bdi").unwrap());
static PRICE_SEL_2: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price del span bdi").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "BestBuyTunisie",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://bestbuytunisie.tn/vente/informatique/pc/pc-portable-tunisie/"),
        (&Section::GamingLaptop, "https://bestbuytunisie.tn/vente/informatique/pc/pc-portable-gamer-tunisie/"),
        (&Section::PC, "https://bestbuytunisie.tn/vente/informatique/pc/pc-de-bureau-tunisie/"),
        (&Section::GamingPc, "https://bestbuytunisie.tn/vente/gaming/pc-gamer-tunisie/"),
        (&Section::PcAllInOne, "https://bestbuytunisie.tn/vente/informatique/pc/pc-tout-en-un-tunisie/"),
        (&Section::Monitor, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/ecran-tunisie/"),
        (&Section::Monitor, "https://bestbuytunisie.tn/vente/gaming/peripheriques-et-accessoires-gamers/ecrans-gamer-tunisie/"),
        (&Section::CPU, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/processeur-tunisie/"),
        (&Section::GPU, "https://bestbuytunisie.tn/vente/gaming/composants/carte-graphique-tunisie/"),
        (&Section::RAM, "https://bestbuytunisie.tn/vente/gaming/composants/barrette-memoire-tunisie/"),
        (&Section::MotherBoard, "https://bestbuytunisie.tn/vente/informatique/composants-pc/carte-mere-pc-tunisie/"),
        (&Section::SSD, "https://bestbuytunisie.tn/vente/informatique/stockage/disque-dur-interne-tunisie/"),
        (&Section::SSD, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/disque-dur-ssd-hdd-mvme-tunisie/"),
        (&Section::Cooler, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/refroidissement-tunisie/"),
        (&Section::PSU, "https://bestbuytunisie.tn/vente/informatique/composants-pc/bloc-dalimentation-pc-tunisie/"),
        (&Section::PSU, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/bloc-dalimentation-tunisie/"),
        (&Section::Case, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/boitier-pc-gamer-tunisie/"),
    ]
};

pub struct BestBuyTunisie;

impl Site for BestBuyTunisie {
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
            .attr("data-lazy-src")
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