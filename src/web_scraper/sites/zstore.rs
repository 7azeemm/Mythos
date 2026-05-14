use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{Html, Selector};
use std::error::Error;

static CONFIG: SiteConfig = SiteConfig {
    name: "ZStore",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("div.product-meta a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("img.attachment-woocommerce_thumbnail").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    sections: &[
        (Section::Laptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-portable/"),
        (Section::GamingLaptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-gaming/"),
        (Section::GamingLaptop, "https://zstore.com.tn/categorie-produit/gaming/laptop-gaming/"),
        (Section::ProLaptop, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-portables/pc-portable-pro/"),
        (Section::PC, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/pc-de-bureau/"),
        (Section::PC, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/mini-pc/"),
        (Section::GamingPC, "https://zstore.com.tn/categorie-produit/gaming/ordinateurs-de-bureau-gaming/"),
        (Section::GamingSetup, "https://zstore.com.tn/categorie-produit/gaming/full-setup-gamer/"),
        (Section::AllInOnePC, "https://zstore.com.tn/categorie-produit/informatique/ordinateurs-de-bureau/pc-tout-en-un/"),
        (Section::Monitor, "https://zstore.com.tn/categorie-produit/gaming/ecran-gaming/"),
        (Section::CPU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/processeur/"),
        (Section::CPU, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/processeur-gaming/"),
        (Section::GPU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/carte-graphique/"),
        (Section::GPU, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/carte-graphique-gaming/"),
        (Section::RAM, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/barrette-memoire/"),
        (Section::MotherBoard, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/cartes-meres/"),
        (Section::MotherBoard, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/cartes-meres-gaming/"),
        (Section::Storage, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/disque-dur-interne/"),
        (Section::PSU, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/bloc-dalimentation/"),
        (Section::Cooler, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/ventilateur-et-refroidisseur/"),
        (Section::Cooler, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/refroidissement/"),
        (Section::Case, "https://zstore.com.tn/categorie-produit/informatique/composants-informatiques-informatique/boitier/"),
        (Section::Case, "https://zstore.com.tn/categorie-produit/gaming/composants-gaming/boitiers-gaming/"),
    ]
};

pub struct ZStore;

impl Site for ZStore {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_page_count(&self, _doc: &Html) -> Result<i32, Box<dyn Error>> {
        Ok(1)
    }

    fn format_url(&self, url: &str, _page: i32) -> String {
        url.to_string()
    }
}