use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "TDiscount",
    web_client_type: WebClientType::Browser,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div article.product-card").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.woo-loop-product__title a").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.mf-product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://tdiscount.tn/categorie-produit/informatique/ordinateur-de-bureau/"),
        (Section::Laptop, "https://tdiscount.tn/categorie-produit/informatique/pc-portable/"),
        (Section::GamingLaptop, "https://tdiscount.tn/categorie-produit/gaming/pc-gamer/"),
        (Section::GamingLaptop, "https://tdiscount.tn/categorie-produit/gaming/pc-portable-gamer/"),
        (Section::Monitor, "https://tdiscount.tn/categorie-produit/informatique/ecran-pc/"),
        (Section::GPU, "https://tdiscount.tn/categorie-produit/gaming/composant-pc-gamer/"),
        (Section::Memory, "https://tdiscount.tn/categorie-produit/informatique/composants-informatique/"),
        (Section::Storage, "https://tdiscount.tn/categorie-produit/informatique/stockage/"),
        (Section::Mouse, "https://tdiscount.tn/categorie-produit/informatique/accessoires-informatique/"),
        (Section::Mouse, "https://tdiscount.tn/categorie-produit/gaming/accessoires-pc-gamer/"),
        (Section::Controller, "https://tdiscount.tn/categorie-produit/gaming/jeu-video-console/"),
        (Section::Smartphone, "https://tdiscount.tn/categorie-produit/telephonie-tablette/smartphone-tunisie/"),
        (Section::Tablet, "https://tdiscount.tn/categorie-produit/telephonie-tablette/tablette/"),
        (Section::Smartwatch, "https://tdiscount.tn/categorie-produit/telephonie-tablette/montre-connectee/"),
        (Section::Television, "https://tdiscount.tn/categorie-produit/tv-image-son/televiseur/"),
    ],
};

pub struct TDiscount;

impl Retailer for TDiscount {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}
