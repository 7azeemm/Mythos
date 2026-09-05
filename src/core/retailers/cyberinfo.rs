use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "CyberInfo",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.item-product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("span#product-availability").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap())),
    page_desc_sel: None,
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://www.cyberinfo.tn/43-pc-de-bureau"),
        (Section::GamingPC, "https://www.cyberinfo.tn/191-pc-de-bureau-gamer-tunisie"),
        (Section::GamingPC, "https://www.cyberinfo.tn/211-full-setup-gamer-tunisie"),
        (Section::AllInOnePC, "https://www.cyberinfo.tn/44-pc-all-in-one"),
        (Section::Laptop, "https://www.cyberinfo.tn/42-pc-portable"),
        (Section::GamingLaptop, "https://www.cyberinfo.tn/190-pc-portable-gamer"),
        (Section::Monitor, "https://www.cyberinfo.tn/45-ecran-pc"),
        (Section::CPU, "https://www.cyberinfo.tn/58-processeur-tunisie"),
        (Section::GPU, "https://www.cyberinfo.tn/60-carte-graphique-tunisie"),
        (Section::Memory, "https://www.cyberinfo.tn/55-barette-memoire-ram-tunisie"),
        (Section::Storage, "https://www.cyberinfo.tn/67-disque-dur-ssd-tunisie"),
        (Section::Storage, "https://www.cyberinfo.tn/65-disque-dur-interne-tunisie"),
        (Section::Motherboard, "https://www.cyberinfo.tn/56-carte-mere-tunisie"),
        (Section::Cooler, "https://www.cyberinfo.tn/50-refroidisseur-pc-tunisie"),
        (Section::Cooler, "https://www.cyberinfo.tn/59-ventilateur-refroidisseur-pc-tunisie"),
        (Section::PowerSupply, "https://www.cyberinfo.tn/193-boite-alimentation-tunisie"),
        (Section::Case, "https://www.cyberinfo.tn/192-boitier-pc"),
        (Section::Mouse, "https://www.cyberinfo.tn/47-souris-pc-tunisie"),
        (Section::Keyboard, "https://www.cyberinfo.tn/195-clavier-pc"),
        (Section::MousePad, "https://www.cyberinfo.tn/48-tapis-de-souris"),
        (Section::Headphones, "https://www.cyberinfo.tn/194-casque-et-ecouteurs-tunisie"),
        (Section::AccessoriesCombo, "https://www.cyberinfo.tn/196-ensemble-clavier-souris"),
        (Section::Console, "https://www.cyberinfo.tn/213-playstaion"),
        (Section::Controller, "https://www.cyberinfo.tn/214-manette"),
        (Section::ConsoleGame, "http://cyberinfo.tn/215-jeux-video"),
        (Section::ConsoleAccessories, "https://www.cyberinfo.tn/216-accessoires-console"),
        (Section::Smartphone, "https://www.cyberinfo.tn/187-smartphone-prix-tunisie"),
        (Section::Smartphone, "https://www.cyberinfo.tn/188-iphone-apple-tunisie"),
        (Section::Tablet, "https://www.cyberinfo.tn/80-tablette-android-tunisie"),
        (Section::Tablet, "https://www.cyberinfo.tn/218-ipad"),
        (Section::Smartwatch, "https://www.cyberinfo.tn/145-montre-connectee-tunisie"),
        (Section::Television, "https://www.cyberinfo.tn/26-televiseur-tunisie"),
    ],
};

pub struct CyberInfo;

impl Retailer for CyberInfo {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }
}
