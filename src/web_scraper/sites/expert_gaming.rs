use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "ExpertGaming",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav ul.page-numbers li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products section.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.thumbnail-wrapper a figure img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: Some(Lazy::new(|| Selector::parse("div.short-description").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::GamingPC, "https://www.expert-gaming.tn/pc-gaming-bureautique/"),
        (Section::GamingSetup, "https://www.expert-gaming.tn/full-setup-gaming/"),
        (Section::GamingLaptop, "https://www.expert-gaming.tn/pc-portable-gaming/"),
        (Section::Monitor, "https://www.expert-gaming.tn/ecran-gaming/"),
        (Section::Monitor, "https://www.expert-gaming.tn/ecrans-professionnelles/"),
        (Section::CPU, "https://www.expert-gaming.tn/processeurs-intel/"),
        (Section::CPU, "https://www.expert-gaming.tn/processeurs-amd/"),
        (Section::GPU, "https://www.expert-gaming.tn/carte-graphique-nvidia/"),
        (Section::GPU, "https://www.expert-gaming.tn/carte-graphique-amd/"),
        (Section::RAM, "https://www.expert-gaming.tn/memoire-vive/"),
        (Section::MotherBoard, "https://www.expert-gaming.tn/carte-mere-intel/"),
        (Section::MotherBoard, "https://www.expert-gaming.tn/carte-mere-amd/"),
        (Section::Storage, "https://www.expert-gaming.tn/stockage/"),
        (Section::Storage, "https://www.expert-gaming.tn/disque-interne-externe/"),
        (Section::PSU, "https://www.expert-gaming.tn/alimentation/"),
        (Section::Case, "https://www.expert-gaming.tn/boitier/"),
        (Section::Cooler, "https://www.expert-gaming.tn/refroidissement-a-eau-watercooling/"),
        (Section::Cooler, "https://www.expert-gaming.tn/refroidissement-a-air-aircooling/"),
        (Section::Cooler, "https://www.expert-gaming.tn/ventilateur-boitier/"),
    ],
};

pub struct ExpertGaming;

impl Site for ExpertGaming {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}