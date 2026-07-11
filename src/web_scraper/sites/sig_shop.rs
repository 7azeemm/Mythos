use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "SigShop",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav ul.page-numbers li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("a.woocommerce-LoopProduct-link").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-short-description").unwrap())),
    page_desc_sel: None,
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://sig-shop.tn/categorie-produit/pc-bureau/"),
        (Section::GamingPC, "https://sig-shop.tn/categorie-produit/gaming-2/pc-bureau-gaming-2/"),
        (Section::AllInOnePC, "https://sig-shop.tn/categorie-produit/pc-bureau/all-in-one/"),
        (Section::Laptop, "https://sig-shop.tn/categorie-produit/pc-portable/"),
        (Section::GamingLaptop, "https://sig-shop.tn/categorie-produit/gaming-2/pc-portable-gaming-2/"),
        (Section::MacBook, "https://sig-shop.tn/categorie-produit/mac-imac/"),
        (Section::Monitor, "https://sig-shop.tn/categorie-produit/accessoires/ecran/"),
        (Section::CPU, "https://sig-shop.tn/categorie-produit/accessoires/processeur/"),
        (Section::GPU, "https://sig-shop.tn/categorie-produit/accessoires/carte-graphique/"),
        (Section::Memory, "https://sig-shop.tn/categorie-produit/accessoires/barrette-memoire/"),
        (Section::Storage, "https://sig-shop.tn/categorie-produit/accessoires/stockage/disque-dur-interne/"),
        (Section::Motherboard, "https://sig-shop.tn/categorie-produit/accessoires/carte-mere-accessoires/"),
        (Section::Case, "https://sig-shop.tn/categorie-produit/accessoires/boitier/"),
        (Section::Mouse, "https://sig-shop.tn/categorie-produit/accessoires/souris/"),
        (Section::Keyboard, "https://sig-shop.tn/categorie-produit/accessoires/clavier/"),
        (Section::Console, "https://sig-shop.tn/brand/sony/"),
        (Section::Smartphone, "https://sig-shop.tn/categorie-produit/telephonie/smartphone/"),
        (Section::Smartphone, "https://sig-shop.tn/categorie-produit/telephonie/smartphone/iphone/"),
        (Section::Tablet, "https://sig-shop.tn/categorie-produit/ipad/"),
        (Section::Tablet, "https://sig-shop.tn/categorie-produit/tablette/"),
        (Section::Smartwatch, "https://sig-shop.tn/categorie-produit/apple-watch/"),
        (Section::Smartwatch, "https://sig-shop.tn/categorie-produit/montre-connectee/"),
        (Section::Television, "https://sig-shop.tn/categorie-produit/televiseur/samsung-televiseur/"),
        (Section::Television, "https://sig-shop.tn/categorie-produit/televiseur/vega/"),
    ]
};

pub struct SigShop;

impl Site for SigShop {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}