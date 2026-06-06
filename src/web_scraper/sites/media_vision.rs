use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "MediaVision",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article[data-id-product]").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("span.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-add-to-cart button[title] span").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-desc-short").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap())),
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
        (Section::Mouse, "https://www.mediavision.tn/237-souris"),
        (Section::Keyboard, "https://www.mediavision.tn/47-clavier"),
        (Section::Keyboard, "https://www.mediavision.tn/238-clavier"),
        (Section::AccessoriesCombo, "https://www.mediavision.tn/53-pack-bundle"),
        (Section::AccessoriesCombo, "https://www.mediavision.tn/241--pack-bundle"),
        (Section::CPU, "https://www.mediavision.tn/65-processeur"),
        (Section::CPU, "https://www.mediavision.tn/260-processeur"),
        (Section::GPU, "https://www.mediavision.tn/66-carte-graphique"),
        (Section::GPU, "https://www.mediavision.tn/262-carte-graphique"),
        (Section::Memory, "https://www.mediavision.tn/57-barrette-memoire"),
        (Section::Memory, "https://www.mediavision.tn/68-barrette-memoire"),
        (Section::Memory, "https://www.mediavision.tn/266-barrette-memoire"),
        (Section::Motherboard, "https://www.mediavision.tn/67-carte-mere"),
        (Section::Motherboard, "https://www.mediavision.tn/261-carte-mere"),
        (Section::Storage, "https://www.mediavision.tn/39-disque-dur-interne"),
        (Section::Storage, "https://www.mediavision.tn/55-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/64-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/263-disque-dur"),
        (Section::Storage, "https://www.mediavision.tn/264-disque-ssd"),
        (Section::Case, "https://www.mediavision.tn/69-boitier"),
        (Section::Case, "https://www.mediavision.tn/268-boitiers"),
        (Section::Cooler, "https://www.mediavision.tn/62-ventilateur"),
        (Section::Cooler, "https://www.mediavision.tn/70-refroidissement-processeur"),
        (Section::Cooler, "https://www.mediavision.tn/265-refroidissement-processeur"),
        (Section::Cooler, "https://www.mediavision.tn/273-ventilateur"),
        (Section::PowerSupply, "https://www.mediavision.tn/71-bloc-alimentation"),
        (Section::PowerSupply, "https://www.mediavision.tn/259-alimentation"),
    ]
};

pub struct MediaVision;

impl Site for MediaVision {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}