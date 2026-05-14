use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "InfoTec",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.wd-product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.wd-entities-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-image-link img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    sections: &[
        (Section::Laptop, "https://infotec.tn/fr/c/ordinateurs-portables/"),
        (Section::GamingLaptop, "https://infotec.tn/fr/c/pc-gamer/"),
        (Section::GamingLaptop, "https://infotec.tn/fr/c/pc-portable-gamer/"),
        (Section::PC, "https://infotec.tn/fr/c/pc-de-bureau/"),
        (Section::GamingPC, "https://infotec.tn/fr/c/ordinateur-gamer/"),
        (Section::GamingPC, "https://infotec.tn/fr/c/ordinateur-gamer-gaming-pc/"),
        (Section::AllInOnePC, "https://infotec.tn/fr/c/pc-tout-en-un/"),
        (Section::Monitor, "https://infotec.tn/fr/c/ecran/"),
        (Section::Monitor, "https://infotec.tn/fr/c/ecran-gamer/"),
        (Section::CPU, "https://infotec.tn/fr/c/processeur/"),
        (Section::CPU, "https://infotec.tn/fr/c/processeur-composant-pc-gamer/"),
        (Section::GPU, "https://infotec.tn/fr/c/carte-graphique/"),
        (Section::GPU, "https://infotec.tn/fr/c/carte-graphique-composant-pc-gamer/"),
        (Section::RAM, "https://infotec.tn/fr/c/barrettes-memoire/"),
        (Section::RAM, "https://infotec.tn/fr/c/barrette-memoire-gamer/"),
        (Section::MotherBoard, "https://infotec.tn/fr/c/carte-mere/"),
        (Section::MotherBoard, "https://infotec.tn/fr/c/carte-mere-composant-pc-gamer/"),
        (Section::Storage, "https://infotec.tn/fr/c/disque-dur-interne/"),
        (Section::Storage, "https://infotec.tn/fr/c/disque-dur/"),
        (Section::Storage, "https://infotec.tn/fr/c/disque-dur-ssd/"),
        (Section::Case, "https://infotec.tn/fr/c/boitier/"),
        (Section::Case, "https://infotec.tn/fr/c/boitier-pc-gamer/"),
        (Section::PSU, "https://infotec.tn/fr/c/bloc-dalimentation/"),
        (Section::PSU, "https://infotec.tn/fr/c/alimentation-pc-gamer/"),
        (Section::Cooler, "https://infotec.tn/fr/c/refroidisseur-processeur-gamer/"),
        (Section::Cooler, "https://infotec.tn/fr/c/ventilateur-gamer/"),
    ]
};

pub struct InfoTec;

impl Site for InfoTec {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}