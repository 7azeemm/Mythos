use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "WikiTN",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products-grid__grid div.product-card--grid").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-card__title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.product-card__top figure img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("p.price span bdi").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("p.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("p.price ins span bdi").unwrap())),
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-availability div[data-stock-status]").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://wiki.tn/ordinateur-de-bureau"),
        (Section::GamingPC, "https://wiki.tn/pc-bureau-gamer"),
        (Section::AllInOnePC, "https://wiki.tn/pc-all-in-one"),
        (Section::AllInOnePC, "https://wiki.tn/imac/"),
        (Section::Laptop, "https://wiki.tn/pc-portables/"),
        (Section::GamingLaptop, "https://wiki.tn/pc-portable-gamer"),
        (Section::MacBook, "https://wiki.tn/macbook"),
        (Section::Monitor, "https://wiki.tn/ecran/"),
        (Section::CPU, "https://wiki.tn/processeur/"),
        (Section::GPU, "https://wiki.tn/carte-graphique"),
        (Section::Memory, "https://wiki.tn/barrette-memoire-composants-maintenance"),
        (Section::Memory, "https://wiki.tn/barrette-memoire-rgb-composants-gamer/"),
        (Section::Storage, "https://wiki.tn/disque-dur-interne/"),
        (Section::Motherboard, "https://wiki.tn/carte-mere/"),
        (Section::Cooler, "https://wiki.tn/ventilateurs-composants-maintenance/"),
        (Section::Cooler, "https://wiki.tn/watercooling/"),
        (Section::Cooler, "https://wiki.tn/ventilateurs-rgb"),
        (Section::PowerSupply, "https://wiki.tn/boite-dalimentation/"),
        (Section::Case, "https://wiki.tn/boitier"),
        (Section::Mouse, "https://wiki.tn/souris-tapis"),
        (Section::Mouse, "https://wiki.tn/souris-gamer/"),
        (Section::Keyboard, "https://wiki.tn/clavier"),
        (Section::Keyboard, "https://wiki.tn/clavier-gaming/"),
        (Section::Headphones, "https://wiki.tn/ecouteurs-et-kit-bluetooth"),
        (Section::Headphones, "https://wiki.tn/ecouteurs-micro-casque/"),
        (Section::Headphones, "https://wiki.tn/casque"),
        (Section::Headphones, "https://wiki.tn/casque-micro/"),
        (Section::GamingChair, "https://wiki.tn/chaise-gaming"),
        (Section::AccessoriesCombo, "https://wiki.tn/ensemble-clavier-et-souris-peripheriques-pc"),
        (Section::AccessoriesCombo, "https://wiki.tn/pack-gaming"),
        (Section::Console, "https://wiki.tn/ps5"),
        (Section::Controller, "https://wiki.tn/manette-de-jeux"),
        (Section::ConsoleGame, "https://wiki.tn/jeux-videos/"),
        (Section::Smartphone, "https://wiki.tn/smartphones"),
        (Section::Smartphone, "https://wiki.tn/iphone"),
        (Section::Tablet, "https://wiki.tn/tablette"),
        (Section::Tablet, "https://wiki.tn/ipad"),
        (Section::Smartwatch, "https://wiki.tn/montre-connectee"),
        (Section::Smartwatch, "https://wiki.tn/bracelet-connecte/"),
        (Section::Television, "https://wiki.tn/tv-led"),
    ],
};

pub struct WikiTN;

impl Retailer for WikiTN {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?_pagination={page}")
    }
}
