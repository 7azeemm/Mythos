use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "CyberInfo",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.item-product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("span#product-availability").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::Laptop, "https://www.cyberinfo.tn/42-pc-portable"),
        (Section::GamingLaptop, "https://www.cyberinfo.tn/190-pc-portable-gamer"),
        (Section::PC, "https://www.cyberinfo.tn/43-pc-de-bureau"),
        (Section::GamingPC, "https://www.cyberinfo.tn/191-pc-de-bureau-gamer-tunisie"),
        (Section::GamingSetup, "https://www.cyberinfo.tn/211-full-setup-gamer-tunisie"),
        (Section::AllInOnePC, "https://www.cyberinfo.tn/44-pc-all-in-one"),
        (Section::Monitor, "https://www.cyberinfo.tn/45-ecran-pc"),
        (Section::CPU, "https://www.cyberinfo.tn/58-processeur-tunisie"),
        (Section::GPU, "https://www.cyberinfo.tn/60-carte-graphique-tunisie"),
        (Section::RAM, "https://www.cyberinfo.tn/55-barette-memoire-ram-tunisie"),
        (Section::MotherBoard, "https://www.cyberinfo.tn/56-carte-mere-tunisie"),
        (Section::Storage, "https://www.cyberinfo.tn/67-disque-dur-ssd-tunisie"),
        (Section::Storage, "https://www.cyberinfo.tn/65-disque-dur-interne-tunisie"),
        (Section::Cooler, "https://www.cyberinfo.tn/50-refroidisseur-pc-tunisie"),
        (Section::Cooler, "https://www.cyberinfo.tn/59-ventilateur-refroidisseur-pc-tunisie"),
        (Section::Case, "https://www.cyberinfo.tn/192-boitier-pc"),
        (Section::PSU, "https://www.cyberinfo.tn/193-boite-alimentation-tunisie"),
    ]
};

pub struct CyberInfo;

impl Site for CyberInfo {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}