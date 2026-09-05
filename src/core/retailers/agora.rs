use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "Agora",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article[data-id-product]").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.thumbnail img").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("span#product-availability").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("p.an_short_description").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap())),
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://agora.tn/fr/16-pc-de-bureau"),
        (Section::GamingPC, "https://agora.tn/fr/603-pc-bureau-gamer-"),
        (Section::Laptop, "https://agora.tn/fr/12-pc-portable"),
        (Section::GamingLaptop, "https://agora.tn/fr/799-pc-portable-gaming"),
        (Section::Monitor, "https://agora.tn/fr/97-ecran-de-bureau"),
        (Section::Monitor, "https://agora.tn/fr/605-ecran-gamer"),
        (Section::CPU, "https://agora.tn/fr/641-processeur-"),
        (Section::GPU, "https://agora.tn/fr/104-carte-graphique"),
        (Section::Memory, "https://agora.tn/fr/103-barrette-memoire"),
        (Section::Storage, "https://agora.tn/fr/113-disque-dur-interne"),
        (Section::Motherboard, "https://agora.tn/fr/109-carte-mere"),
        (Section::Cooler, "https://agora.tn/fr/590-ventilateur"),
        (Section::PowerSupply, "https://agora.tn/fr/637-boitier-bloc-d-alimentation"),
        (Section::Case, "https://agora.tn/fr/584-boitier-"),
        (Section::Mouse, "https://agora.tn/fr/122-clavier-souris-tapis"),
        (Section::Mouse, "https://agora.tn/fr/610-souris-et-tapis-gaming"),
        (Section::Keyboard, "https://agora.tn/fr/606-claviers-gaming"),
        (Section::Headphones, "https://agora.tn/fr/382-casques-et-ecouteurs"),
        (Section::Headphones, "https://agora.tn/fr/40-ecouteur"),
        (Section::Headphones, "https://agora.tn/fr/941--casque"),
        (Section::Headphones, "https://agora.tn/fr/608-casques-et-ecouteurs-gaming"),
        (Section::GamingChair, "https://agora.tn/fr/601-chaise-gaming"),
        (Section::Controller, "https://agora.tn/fr/897-manette"),
        (Section::Smartphone, "https://agora.tn/fr/225-smartphones"),
        (Section::Smartphone, "https://agora.tn/fr/904-iphone"),
        (Section::Tablet, "https://agora.tn/fr/932-tablette-android"),
        (Section::Tablet, "https://agora.tn/fr/929-ipad"),
        (Section::Smartwatch, "https://agora.tn/fr/692-montre-connectee"),
        (Section::Television, "https://agora.tn/fr/78-tv"),
        (Section::Television, "https://agora.tn/fr/949-tv-hospitality"),
    ],
};

pub struct Agora;

impl Retailer for Agora {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }
}
