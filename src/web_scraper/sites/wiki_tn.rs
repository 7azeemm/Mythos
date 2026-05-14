use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "WikiTN",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products-grid__grid div.product-card--grid").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-card__title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.product-card__top figure img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("p.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("p.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("p.price ins span bdi").unwrap())),
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-availability div[data-stock-status]").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.decriptions-short").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::Laptop, "https://wiki.tn/pc-portables/"),
        (Section::GamingLaptop, "https://wiki.tn/pc-portable-gamer"),
        (Section::PC, "https://wiki.tn/ordinateur-de-bureau"),
        (Section::GamingPC, "https://wiki.tn/pc-gamer/"),
        (Section::AllInOnePC, "https://wiki.tn/pc-all-in-one/"),
        (Section::CPU, "https://wiki.tn/processeur/"),
        (Section::GPU, "https://wiki.tn/carte-graphique"),
        (Section::RAM, "https://wiki.tn/barrette-memoire-composants-maintenance"),
        (Section::RAM, "https://wiki.tn/barrette-memoire-rgb-composants-gamer/"),
        (Section::MotherBoard, "https://wiki.tn/carte-mere/"),
        (Section::Storage, "https://wiki.tn/disque-dur-interne/"),
        (Section::Cooler, "https://wiki.tn/ventilateurs-composants-maintenance/"),
        (Section::Cooler, "https://wiki.tn/watercooling/"),
        (Section::Cooler, "https://wiki.tn/ventilateurs-rgb"),
        (Section::Monitor, "https://wiki.tn/ecran/"),
        (Section::Case, "https://wiki.tn/boitier"),
        (Section::PSU, "https://wiki.tn/boite-dalimentation/"),
    ]
};

pub struct WikiTN;

impl Site for WikiTN {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?_pagination={page}")
    }
}