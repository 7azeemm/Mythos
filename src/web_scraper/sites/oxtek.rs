use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "OXTek",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article.product-miniature").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.product-price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-availability span").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-description-short").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.product-description").unwrap())),
    sections: &[
        (Section::PC, "https://www.technopro-online.com/pc-de-bureau.html"),
        (Section::GamingPC, "https://www.technopro-online.com/pc-gamer.html"),
        (Section::AllInOnePC, "https://www.technopro-online.com/prix-pc-de-bureau-tout-en-un-tunisie.html"),
        (Section::AllInOnePC, "https://www.technopro-online.com/prix-apple-imac-tunisie.html"),
        (Section::AllInOnePC, "https://www.technopro-online.com/prix-mac-pro-tunisie.html"),
        (Section::Laptop, "https://www.technopro-online.com/prix-pc-portable-hp-dell-asus-lenovo-acer-Tunisie.html"),
        (Section::GamingLaptop, "https://www.technopro-online.com/pc-portable-gamer.html"),
        (Section::MacBook, "https://www.technopro-online.com/prix-macbook-tunisie.html"),
        (Section::Monitor, "https://www.technopro-online.com/prix-ecran-ordinateur-moniteur-samsung-dell-hp-lenovo-acer-lg-tunisie.html"),
        (Section::Monitor, "https://www.technopro-online.com/-ecran-gamer.html"),
        (Section::Mouse, "https://www.technopro-online.com/prix-souris-tunisie.html"),
        (Section::Mouse, "https://www.technopro-online.com/souris-gamer.html"),
        (Section::Keyboard, "https://www.technopro-online.com/claviers-multimedia.html"),
        (Section::Keyboard, "https://www.technopro-online.com/clavier-gamer-.html"),
        (Section::AccessoriesCombo, "https://www.technopro-online.com/ensemble-claviersouris.html"),
        (Section::AccessoriesCombo, "https://www.technopro-online.com/ensemble-clavier-souris-gamer.html"),
        (Section::CPU, "https://www.technopro-online.com/processeurs.html"),
        (Section::GPU, "https://www.technopro-online.com/cartes-graphiques-msi-asus-macy-tunisie.html"),
        (Section::Memory, "https://www.technopro-online.com/barrette-memoire-pour-pc-de-bureau.html"),
        (Section::Memory, "https://www.technopro-online.com/barrette-memoire-pour-serveur-tunisie.html"),
        (Section::Motherboard, "https://www.technopro-online.com/carte-mere-pour-pc-de-bureau-.html"),
        (Section::Storage, "https://www.technopro-online.com/disques-durs-internes.html"),
        (Section::Storage, "https://www.technopro-online.com/disque-dur-ssd-tunisie.html"),
        (Section::PowerSupply, "https://www.technopro-online.com/bloc-d-alimentation-.html"),
        (Section::Case, "https://www.technopro-online.com/boitier-pc-gamer-.html"),
        (Section::Cooler, "https://www.technopro-online.com/ventilateur-gamer-.html"),
        (Section::Cooler, "https://www.technopro-online.com/prix-systemes-de-refroidissement-tunisie.html"),
    ]
};

pub struct OXTek;

impl Site for OXTek {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}