use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "MediaVision",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article[data-id-product]").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.add-to-cart-btn button[title]").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("span.product-descriptions").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap())),
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://www.mediavision.tn/29-pc-de-bureau"),
        (Section::GamingPC, "https://www.mediavision.tn/30-pc-gamer"),
        (Section::GamingPC, "https://www.mediavision.tn/234-pc-de-bureau-gamer"),
        (Section::AllInOnePC, "https://www.mediavision.tn/31-pc-tout-en-un"),
        (Section::AllInOnePC, "https://www.mediavision.tn/33-imac"),
        (Section::Laptop, "https://www.mediavision.tn/24-pc-portable"),
        (Section::Laptop, "https://www.mediavision.tn/362-pc-portable-professionnel-"),
        (Section::Laptop, "https://www.mediavision.tn/363-pc-ultrabook"),
        (Section::GamingLaptop, "https://www.mediavision.tn/25-pc-portable-gamer"),
        (Section::GamingLaptop, "https://www.mediavision.tn/233-pc-portable-gamer"),
        (Section::MacBook, "https://www.mediavision.tn/366-macbook"),
        (Section::Monitor, "https://www.mediavision.tn/34-ecran"),
        (Section::Monitor, "https://www.mediavision.tn/235-ecran"),
        (Section::CPU, "https://www.mediavision.tn/65-processeur"),
        (Section::CPU, "https://www.mediavision.tn/260-processeur"),
        (Section::GPU, "https://www.mediavision.tn/66-carte-graphique"),
        (Section::GPU, "https://www.mediavision.tn/262-carte-graphique"),
        (Section::Memory, "https://www.mediavision.tn/57-barrette-memoire"),
        (Section::Memory, "https://www.mediavision.tn/68-barrette-memoire"),
        (Section::Memory, "https://www.mediavision.tn/266-barrette-memoire"),
        (Section::Storage, "https://www.mediavision.tn/39-disque-dur-interne"),
        (Section::Storage, "https://www.mediavision.tn/55-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/64-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/263-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/264-disque-ssd"),
        (Section::Motherboard, "https://www.mediavision.tn/67-carte-mere"),
        (Section::Motherboard, "https://www.mediavision.tn/261-carte-mere"),
        (Section::Cooler, "https://www.mediavision.tn/62-ventilateur"),
        (Section::Cooler, "https://www.mediavision.tn/70-refroidissement-processeur"),
        (Section::Cooler, "https://www.mediavision.tn/265-refroidissement-processeur"),
        (Section::Cooler, "https://www.mediavision.tn/273-ventilateur"),
        (Section::PowerSupply, "https://www.mediavision.tn/71-bloc-alimentation"),
        (Section::PowerSupply, "https://www.mediavision.tn/259-alimentation"),
        (Section::Case, "https://www.mediavision.tn/69-boitier"),
        (Section::Case, "https://www.mediavision.tn/268-boitiers"),
        (Section::Mouse, "https://www.mediavision.tn/237-souris"),
        (Section::Keyboard, "https://www.mediavision.tn/47-clavier"),
        (Section::Keyboard, "https://www.mediavision.tn/238-clavier"),
        (Section::MousePad, "https://www.mediavision.tn/240-tapis-souris"),
        (Section::Headphones, "https://www.mediavision.tn/205-casque-kit"),
        (Section::Headphones, "https://www.mediavision.tn/48-micro-casques"),
        (Section::AccessoriesCombo, "https://www.mediavision.tn/53-pack-bundle"),
        (Section::AccessoriesCombo, "https://www.mediavision.tn/241--pack-bundle"),
    ],
};

pub struct MediaVision;

impl Retailer for MediaVision {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }
}
