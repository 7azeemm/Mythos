use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "JMB",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.thumbnail-wrapper img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    sections: &[
        (Section::PC, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/pc-bureau-tunisie/"),
        (Section::GamingPC, "https://jmb.com.tn/gaming/gaming-pc/pc-de-bureau-gamer/"),
        (Section::AllInOnePC, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/pc-all-in-one/"),
        (Section::Laptop, "https://jmb.com.tn/informatique-tunisie/ordinateurs-portable/"),
        (Section::GamingLaptop, "https://jmb.com.tn/gaming/gaming-pc/pc-portable-gamer/"),
        (Section::MacBook, "https://jmb.com.tn/informatique-tunisie/ordinateurs-portable/pc-mac-tunisie/"),
        (Section::Monitor, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/ecran/"),
        (Section::Monitor, "https://jmb.com.tn/gaming/gaming-pc/ecran-gamer-pc-gaming-en-ligne-tunisie/"),
        (Section::Mouse, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/souris-tapis-gamer-tunisie/"),
        (Section::Keyboard, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/clavier-gamer-accessoires-gamer-gaming/"),
        (Section::AccessoriesCombo, "https://jmb.com.tn/informatique-tunisie/peripheriques-et-accessoires/clavier-souris/"),
        (Section::CPU, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/processeur/"),
        (Section::GPU, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/carte-graphique/"),
        (Section::GPU, "https://jmb.com.tn/gaming/composant-gamer/carte-graphique-gamer-composants-gamer-gaming/"),
        (Section::Memory, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/carte-mere-barrette-memoire/"),
        (Section::Motherboard, "https://jmb.com.tn/gaming/composant-gamer/carte-mere-gamer-composants-gamer-gaming/"),
        (Section::Storage, "https://jmb.com.tn/informatique-tunisie/stockage/disque-dur-interne/"),
        (Section::Storage, "https://jmb.com.tn/informatique-tunisie/stockage/disque-dur-ssd/"),
        (Section::Case, "https://jmb.com.tn/gaming/composant-gamer/boitier-gamer/"),
        (Section::PowerSupply, "https://jmb.com.tn/gaming/composant-gamer/bloc-dalimentation-gamer-composants-gamer-gaming/"),
    ]
};

pub struct JMB;

impl Site for JMB {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}")
    }
}