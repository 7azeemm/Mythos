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

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h1.product-item-name").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.card-body a.product-item-link").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-item-photo a[href]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.search-short-description").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.stock span").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.price-box span.final-price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.price-box span.original-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "Mytek",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.custom-pagination ul.pagination li.page-item").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div#product-list-container div#seo-product-data div[data-id]").unwrap()),
    sections: &[
        (&Section::Laptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-portable.html"),
        (&Section::GamingLaptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-gamer.html"),
        (&Section::ProLaptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-portable-pro.html"),
        (&Section::ProLaptop, "https://www.mytek.tn/informatique/ordinateurs-portables/mac.html"),
        (&Section::ProLaptop, "https://www.mytek.tn/informatique/ordinateurs-portables/ultrabook.html"),
        (&Section::PC, "https://www.mytek.tn/informatique/ordinateur-de-bureau/pc-de-bureau.html"),
        (&Section::GamingPc, "https://www.mytek.tn/informatique/ordinateur-de-bureau/ordinateur-gamer.html"),
        (&Section::PcAllInOne, "https://www.mytek.tn/informatique/ordinateur-de-bureau/pc-tout-en-un.html"),
        (&Section::Monitor, "https://www.mytek.tn/informatique/ordinateur-de-bureau/ecran.html"),
        (&Section::Monitor, "https://www.mytek.tn/gaming/peripheriques-et-accessoires-gamers/ecran-gamer.html"),
        (&Section::SSD, "https://www.mytek.tn/informatique/stockage/disque-dur.html"),
        (&Section::CPU, "https://www.mytek.tn/informatique/composants-informatique/processeur.html"),
        (&Section::GPU, "https://www.mytek.tn/informatique/composants-informatique/carte-graphique.html"),
        (&Section::RAM, "https://www.mytek.tn/gaming/composant-pc-gamer/barrette-memoire-gamer.html"),
        (&Section::RAM, "https://www.mytek.tn/informatique/composants-informatique/barrettes-memoire.html"),
        (&Section::MotherBoard, "https://www.mytek.tn/informatique/composants-informatique/carte-mere.html"),
        (&Section::Cooler, "https://www.mytek.tn/gaming/composant-pc-gamer/refroidisseur-processeur-gamer.html"),
        (&Section::Case, "https://www.mytek.tn/informatique/composants-informatique/boitier.html"),
        (&Section::Case, "https://www.mytek.tn/gaming/composant-pc-gamer/boitier-pc-gamer.html"),
        (&Section::PSU, "https://www.mytek.tn/gaming/composant-pc-gamer/alimentation-pc-gamer.html"),
        (&Section::PSU, "https://www.mytek.tn/informatique/composants-informatique/bloc-d-alimentation.html"),
    ]
};

pub struct Mytek;

impl Site for Mytek {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.value().attr("data-url").ok_or("url not found")?.to_string();
        let title = element.value().attr("data-name").ok_or("title not found")?.to_string();
        let status = element.value().attr("data-erpstock").ok_or("status not found")?.to_string();
        let price = element.value().attr("data-price").map(|p| parse_price(p)).ok_or("price not found")??;
        let final_price = element.value().attr("data-final-price").map(|p| parse_price(p)).ok_or("final_price not found")??;

        let regular_price = match price == final_price {
            false => Some(price),
            true => None,
        };

        let image = element.value().attr("data-image")
            .map(|s| format!("https://www.mytek.tn/media/catalog/product{s}"))
            .ok_or("image not found")?;

        let description = match section.requires_description() {
            false => vec![],
            true => {
                todo!()
            }
        };

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title,
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status == "En stock",
            price: final_price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }
}