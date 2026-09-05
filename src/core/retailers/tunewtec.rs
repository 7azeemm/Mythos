use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::core::tracking::scrape_error::PaginationError;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::{Html, Selector};

static CONFIG: RetailerConfig = RetailerConfig {
    name: "TunewTec",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products section.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.thumbnail-wrapper img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-label span.out-of-stock").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.short-description").unwrap())),
    page_desc_sel: None,
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/pc-de-bureau/"),
        (Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/mini-pc/"),
        (Section::PC, "https://tunewtec.com/c/ordinateur-de-bureau/station-de-travail/"),
        (Section::GamingPC, "https://tunewtec.com/c/gaming/pc-gaming/"),
        (Section::GamingPC, "https://tunewtec.com/c/ordinateur-de-bureau/pc-gamer/"),
        (Section::AllInOnePC, "https://tunewtec.com/c/ordinateur-de-bureau/pc-all-in-one/"),
        (Section::AllInOnePC, "http://tunewtec.com/c/ordinateur-de-bureau/imac/"),
        (Section::Laptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable/"),
        (Section::Laptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable-professionnel/"),
        (Section::GamingLaptop, "https://tunewtec.com/c/ordinateur-portable/pc-portable-gamer/"),
        (Section::GamingLaptop, "https://tunewtec.com/c/gaming/pc-portable-gamer-gaming/"),
        (Section::MacBook, "https://tunewtec.com/c/ordinateur-portable/macbook-macbook-pro/"),
        (Section::Monitor, "https://tunewtec.com/c/gaming/ecran-gaming/"),
        (Section::Monitor, "https://tunewtec.com/c/ordinateur-de-bureau/ecran/"),
        (Section::CPU, "https://tunewtec.com/c/composant-pc-de-bureau/processeur/"),
        (Section::GPU, "https://tunewtec.com/c/composant-pc-de-bureau/cartes-graphiques/"),
        (Section::Memory, "https://tunewtec.com/c/composant-pc-de-bureau/barrettes-memoire-composant-pc-de-bureau/"),
        (Section::Memory, "https://tunewtec.com/c/composant-pc-portable/barrettes-memoire/"),
        (Section::Memory, "https://tunewtec.com/c/gaming/composant-pc-gamer/memoire-gamer/"),
        (Section::Storage, "https://tunewtec.com/c/composant-pc-de-bureau/disque-ssd-composant-pc-de-bureau/"),
        (Section::Storage, "https://tunewtec.com/c/composant-pc-portable/disque-hdd-composant-pc-portable/"),
        (Section::Storage, "https://tunewtec.com/c/composant-pc-portable/disque-ssd-composant-pc-portable/"),
        (Section::Storage, "https://tunewtec.com/c/composant-pc-de-bureau/disque-hdd-composant-pc-de-bureau/"),
        (Section::Motherboard, "https://tunewtec.com/c/composant-pc-de-bureau/carte-mere/"),
        (Section::Motherboard, "https://tunewtec.com/c/gaming/composant-pc-gamer/carte-mere-gamer/"),
        (Section::Cooler, "https://tunewtec.com/c/composant-pc-de-bureau/refroidissement-pc/"),
        (Section::Cooler, "https://tunewtec.com/c/gaming/composant-pc-gamer/ventilateur-gamer/"),
        (Section::PowerSupply, "https://tunewtec.com/c/composant-pc-de-bureau/alimentation-pc/"),
        (Section::PowerSupply, "https://tunewtec.com/c/gaming/composant-pc-gamer/alimentation-gamer/"),
        (Section::Case, "https://tunewtec.com/c/composant-pc-de-bureau/boitier-pc/"),
        (Section::Case, "https://tunewtec.com/c/gaming/composant-pc-gamer/boitier-gamer/"),
        (Section::Mouse, "https://tunewtec.com/c/peripheriques-pc/souris/"),
        (Section::Mouse, "https://tunewtec.com/c/gaming/peripheriques-et-accessoires/souris-gamer/"),
        (Section::Keyboard, "https://tunewtec.com/c/composant-pc-portable/clavier-composant-pc-portable/"),
        (Section::Keyboard, "https://tunewtec.com/c/gaming/peripheriques-et-accessoires/clavier-gamer/"),
        (Section::MousePad, "https://tunewtec.com/c/peripheriques-pc/tapis/"),
        (Section::MousePad, "https://tunewtec.com/c/gaming/peripheriques-et-accessoires/tapis-gamer/"),
        (Section::Headphones, "https://tunewtec.com/c/peripheriques-pc/casque-et-micro/"),
        (Section::Headphones, "https://tunewtec.com/c/image-et-son/son/casque/"),
        (Section::Headphones, "https://tunewtec.com/c/gaming/peripheriques-et-accessoires/casque-gamer/"),
        (Section::AccessoriesCombo, "https://tunewtec.com/c/peripheriques-pc/clavier-et-souris/"),
        (Section::Console, "https://tunewtec.com/c/gaming/console-de-jeux/"),
        (Section::Controller, "https://tunewtec.com/c/gaming/peripheriques-et-accessoires/manettes/"),
        (Section::Smartphone, "https://tunewtec.com/c/iphone/"),
        (Section::Smartphone, "https://tunewtec.com/c/telephonie/mobile/smartphone-android/"),
        (Section::Tablet, "http://tunewtec.com/c/telephonie/tablettes/tablettes-tunisie/"),
        (Section::Tablet, "https://tunewtec.com/c/telephonie/tablettes/ipad-tunisie/"),
        (Section::Smartwatch, "https://tunewtec.com/c/telephonie/montre-connectee/"),
        (Section::Television, "https://tunewtec.com/c/image-et-son/television/tv/"),
    ],
};

pub struct TunewTec;

impl Retailer for TunewTec {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn parse_page_count(&self, _doc: &Html) -> Result<i32, PaginationError> {
        Ok(1)
    }

    fn format_url(&self, url: &str, _page: i32) -> String {
        format!("{url}?per_page=400")
    }
}
