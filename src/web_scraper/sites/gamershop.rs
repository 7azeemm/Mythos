use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "GamerShop",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product_name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img.product_image").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-quantities label").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-des").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::GamingPC, "https://gamershop.tn/203-pc-gamer-tunisie"),
        (Section::GamingLaptop, "https://gamershop.tn/204-pc-portable-gamer"),
        (Section::CPU, "https://gamershop.tn/399-processeur-intel"),
        (Section::CPU, "https://gamershop.tn/910-processeur-amd"),
        (Section::GPU, "https://gamershop.tn/397-carte-graphique"),
        (Section::RAM, "https://gamershop.tn/398-barrette-memoire"),
        (Section::MotherBoard, "https://gamershop.tn/394-carte-mere-intel"),
        (Section::MotherBoard, "https://gamershop.tn/911-carte-mere-amd"),
        (Section::Storage, "https://gamershop.tn/395-disque-dur-ssd"),
        (Section::Case, "https://gamershop.tn/393-boitier"),
        (Section::PSU, "https://gamershop.tn/724-bloc-d-alimentation"),
        (Section::Cooler, "https://gamershop.tn/936-watercooling"),
        (Section::Cooler, "https://gamershop.tn/937-aircooling"),
        (Section::Monitor, "https://gamershop.tn/913-ecran"),
    ]
};

pub struct GamerShop;

impl Site for GamerShop {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}