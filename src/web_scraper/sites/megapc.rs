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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("p.text-skin-base").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.card-img-container img").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.inline-block").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("del.text-sm").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "MegaPC",
    web_client_type: WebClientType::Browser,
    nav_selector: Lazy::new(|| Selector::parse("button.rounded-md.bg-gray-200").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("article.product-card").unwrap()),
    sections: &[
        (&Section::GamingPc, "https://megapc.tn/shop/ORDINATEURS/PC%20GAMER"),
        (&Section::GamingPc, "https://megapc.tn/shop/ORDINATEURS/LEGENDARY"),
        (&Section::GamingPc, "https://megapc.tn/shop/search/PREBUILD"),
        (&Section::GamingSetup, "https://megapc.tn/shop/ORDINATEURS/FULL%20SETUP"),
        (&Section::PcAllInOne, "https://megapc.tn/shop/ORDINATEURS/PC%20TOUT%20EN%20UN"),
        (&Section::PC, "https://megapc.tn/shop/ORDINATEURS/BAREBONE"),
        (&Section::PC, "https://megapc.tn/shop/ORDINATEURS/PRO%20PC"),
        (&Section::GamingLaptop, "https://megapc.tn/shop/PC%20PORTABLE/PC%20PORTABLE%20GAMER"),
        (&Section::ProLaptop, "https://megapc.tn/shop/PC%20PORTABLE/PC%20PORTABLE%20PRO"),
        (&Section::Monitor, "https://megapc.tn/shop/ECRANS/ECRANS%20GAMING"),
        (&Section::Monitor, "https://megapc.tn/shop/ECRANS/ECRANS%20PRO"),
        (&Section::CPU, "https://megapc.tn/shop/COMPOSANTS/PROCESSEUR"),
        (&Section::GPU, "https://megapc.tn/shop/COMPOSANTS/CARTE%20GRAPHIQUE"),
        (&Section::RAM, "https://megapc.tn/shop/COMPOSANTS/BARETTE%20M%C3%89MOIRE"),
        (&Section::MotherBoard, "https://megapc.tn/shop/COMPOSANTS/CARTE%20M%C3%88RE"),
        (&Section::SSD, "https://megapc.tn/shop/STOCKAGE/DISQUE-SSD"),
        (&Section::SSD, "https://megapc.tn/shop/STOCKAGE/DISQUE-NVME"),
        (&Section::HDD, "https://megapc.tn/shop/STOCKAGE/DISQUE-HDD"),
        (&Section::Cooler, "https://megapc.tn/shop/COMPOSANTS/REFROIDISSEMENT"),
        (&Section::PSU, "https://megapc.tn/shop/COMPOSANTS/ALIMENTATION"),
        (&Section::Case, "https://megapc.tn/shop/COMPOSANTS/BOITIER"),
    ]
};

pub struct MegaPC;

impl Site for MegaPC {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.select(&URL_SEL).next().ok_or("url not found")?;
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?.get_text();
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let in_stock = true;

        let description = match section.requires_description() {
            false => vec![],
            true => todo!(),
        };

        let url = url
            .value()
            .attr("href")
            .map(|s| format!("https://megapc.tn{s}"))
            .ok_or("product url not found")?;

        let image = image
            .value()
            .attr("src")
            .map(|s| format!("https://megapc.tn{s}"))
            .ok_or("image url not found")?
            .to_string();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

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