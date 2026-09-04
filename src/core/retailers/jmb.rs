use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
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
    empty_page_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-no-products-found").unwrap())),
    sections: &[
        (Section::PC, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/pc-bureau-tunisie/"),
        (Section::GamingPC, "https://jmb.com.tn/gaming/gaming-pc/pc-de-bureau-gamer/"),
        (Section::AllInOnePC, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/pc-all-in-one/"),
        (Section::Laptop, "https://jmb.com.tn/informatique-tunisie/ordinateurs-portable/"),
        (Section::GamingLaptop, "https://jmb.com.tn/gaming/gaming-pc/pc-portable-gamer/"),
        (Section::MacBook, "https://jmb.com.tn/informatique-tunisie/ordinateurs-portable/pc-mac-tunisie/"),
        (Section::Monitor, "https://jmb.com.tn/informatique-tunisie/ordinateurs-de-bureau/ecran/"),
        (Section::Monitor, "https://jmb.com.tn/gaming/gaming-pc/ecran-gamer-pc-gaming-en-ligne-tunisie/"),
        (Section::CPU, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/processeur/"),
        (Section::GPU, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/carte-graphique/"),
        (Section::GPU, "https://jmb.com.tn/gaming/composant-gamer/carte-graphique-gamer-composants-gamer-gaming/"),
        (Section::Memory, "https://jmb.com.tn/informatique-tunisie/composants-informatiques/carte-mere-barrette-memoire/"),
        (Section::Storage, "https://jmb.com.tn/informatique-tunisie/stockage/disque-dur-interne/"),
        (Section::Storage, "https://jmb.com.tn/informatique-tunisie/stockage/disque-dur-ssd/"),
        (Section::Motherboard, "https://jmb.com.tn/gaming/composant-gamer/carte-mere-gamer-composants-gamer-gaming/"),
        (Section::PowerSupply, "https://jmb.com.tn/gaming/composant-gamer/bloc-dalimentation-gamer-composants-gamer-gaming/"),
        (Section::Case, "https://jmb.com.tn/gaming/composant-gamer/boitier-gamer/"),
        (Section::Mouse, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/souris-tapis-gamer-tunisie/"),
        (Section::Keyboard, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/clavier-gamer-accessoires-gamer-gaming/"),
        (Section::Headphones, "https://jmb.com.tn/informatique-tunisie/peripheriques-et-accessoires/casques/"),
        (Section::Headphones, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/casque-gamer-accessoires-gamer-gaming/"),
        (Section::GamingChair, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/chaise-gamer-tunisie/"),
        (Section::AccessoriesCombo, "https://jmb.com.tn/informatique-tunisie/peripheriques-et-accessoires/clavier-souris/"),
        (Section::Console, "https://jmb.com.tn/gaming/console-jeux/playstation-tunisie/"),
        (Section::Controller, "https://jmb.com.tn/gaming/accessoires-gamer-gaming-en-tunisie/manettes-accessoires-gamer-gaming/"),
        (Section::ConsoleGame, "https://jmb.com.tn/gaming/console-jeux/games-console-de-jeux-gaming/"),
        (Section::Smartphone, "https://jmb.com.tn/telephonie/smartphone-mobile/smartphone-tunisie/"),
        (Section::Smartphone, "https://jmb.com.tn/telephonie/smartphone-mobile/iphone/"),
        (Section::Tablet, "https://jmb.com.tn/telephonie/tablette/tablette-android/"),
        (Section::Tablet, "https://jmb.com.tn/telephonie/tablette/ipad/"),
        (Section::Smartwatch, "https://jmb.com.tn/telephonie/accessoires-telephones/smartwatch/"),
        (Section::Television, "https://jmb.com.tn/image-et-son/televiseur/led-tv-tunisie/"),
        (Section::Television, "https://jmb.com.tn/image-et-son/televiseur/smart-tv-tunisie/"),
    ],
};

pub struct JMB;

impl Retailer for JMB {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}")
    }
}
