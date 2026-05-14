use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "SpaceNet",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product_name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("img.product_image").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-quantities label").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.decriptions-short").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::Laptop, "https://spacenet.tn/18-ordinateur-portable"),
        (Section::GamingLaptop, "https://spacenet.tn/204-pc-portable-gamer-tunisie"),
        (Section::ProLaptop, "https://spacenet.tn/321-pc-portables-pro-tunisie"),
        (Section::PC, "https://spacenet.tn/73-ordinateur-bureau-tunisie"),
        (Section::AllInOnePC, "https://spacenet.tn/80-pc-tout-en-un-tunisie"),
        (Section::GamingPC, "https://spacenet.tn/205-ordinateur-de-bureau-gamer-tunisie"),
        (Section::GamingSetup, "https://spacenet.tn/1390-setup-gaming"),
        (Section::Monitor, "https://spacenet.tn/388-ecran-gamer-tunisie"),
        (Section::Monitor, "https://spacenet.tn/1142-ecrans-professionnels"),
        (Section::CPU, "https://spacenet.tn/399-processeur"),
        (Section::GPU, "https://spacenet.tn/397-cartes-graphiques"),
        (Section::RAM, "https://spacenet.tn/398-memoires-ram"),
        (Section::MotherBoard, "https://spacenet.tn/394-cartes-meres"),
        (Section::Storage, "https://spacenet.tn/395-disque-dur-ssd-hdd-tunisie"),
        (Section::Case, "https://spacenet.tn/393-boitier"),
        (Section::PSU, "https://spacenet.tn/724-bloc-d-alimentation"),
        (Section::Cooler, "https://spacenet.tn/726-ventilateur"),
        (Section::Cooler, "https://spacenet.tn/744-refroidisseur-pc-bureau"),
    ],
};

pub struct SpaceNet;

impl Site for SpaceNet {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}