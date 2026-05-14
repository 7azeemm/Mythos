use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "TDiscount",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.woo-loop-product__title a").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.mf-product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: None,
    desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::PC, "https://tdiscount.tn/categorie-produit/informatique/ordinateur-de-bureau/"),
        (Section::Laptop, "https://tdiscount.tn/categorie-produit/informatique/pc-portable/"),
        (Section::GamingLaptop, "https://tdiscount.tn/categorie-produit/gaming/pc-gamer/"),
        (Section::GamingLaptop, "https://tdiscount.tn/categorie-produit/gaming/pc-portable-gamer/"),
        // (Section::RAM, "https://tdiscount.tn/categorie-produit/informatique/composants-informatique/"),
        // (Section::GPU, "https://tdiscount.tn/categorie-produit/gaming/composant-pc-gamer/"),
        (Section::Monitor, "https://tdiscount.tn/categorie-produit/informatique/ecran-pc/"),
    ]
};

pub struct TDiscount;

impl Site for TDiscount {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
    
    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}