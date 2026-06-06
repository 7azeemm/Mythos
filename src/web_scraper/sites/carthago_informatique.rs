use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "CarthagoInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.woocommerce-loop-product__title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.xts-product-image img").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: Some(Lazy::new(|| Selector::parse("div.berocket_better_labels span b[style]").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    sections: &[
        (Section::PC, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-de-bureau/"),
        (Section::GamingPC, "https://carthagoinformatique.tn/categorie-produit/gaming/pc-gamer/"),
        (Section::AllInOnePC, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-tout-en-un/"),
        (Section::AllInOnePC, "https://carthagoinformatique.tn/categorie-produit/informatique/ordinateur-de-bureau/imac/"),
        (Section::Laptop, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-portable/"),
        (Section::GamingLaptop, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/pc-portable-gamer/"),
        (Section::MacBook, "https://carthagoinformatique.tn/categorie-produit/informatique/pc/mac/"),
        (Section::Monitor, "https://carthagoinformatique.tn/categorie-produit/informatique/accessoires-ordinateur/ecran/"),
        (Section::Mouse, "https://carthagoinformatique.tn/categorie-produit/informatique/accessoires-ordinateur/souris/"),
        (Section::Mouse, "https://carthagoinformatique.tn/categorie-produit/gaming/accessoires-gaming/souris-gaming/"),
        (Section::Keyboard, "https://carthagoinformatique.tn/categorie-produit/informatique/accessoires-ordinateur/claviers/"),
        (Section::Keyboard, "https://carthagoinformatique.tn/categorie-produit/gaming/accessoires-gaming/clavier-gaming/"),
        (Section::AccessoriesCombo, "https://carthagoinformatique.tn/categorie-produit/informatique/accessoires-ordinateur/ensemble-clavier-souris/"),
        (Section::CPU, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/processeur/"),
        (Section::GPU, "https://carthagoinformatique.tn/categorie-produit/gaming/composants/carte-graphique/"),
        (Section::Motherboard, "https://carthagoinformatique.tn/categorie-produit/informatique/composants-pc/carte-mere-pc/"),
        (Section::Memory, "https://carthagoinformatique.tn/categorie-produit/gaming/composants/barrette-memoire/"),
        (Section::Storage, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/disque-dur-ssd-hdd-mvme/"),
        (Section::Storage, "https://carthagoinformatique.tn/categorie-produit/informatique/stockage/disque-dur-interne/"),
        (Section::Cooler, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/refroidissement/"),
        (Section::PowerSupply, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/bloc-dalimentation/"),
        (Section::PowerSupply, "https://carthagoinformatique.tn/categorie-produit/informatique/composants-pc/bloc-dalimentation-pc/"),
        (Section::Case, "https://carthagoinformatique.tn/categorie-produit/gaming/composant-pc-gamer/boitier-pc-gamer/"),
    ],
};

pub struct CarthagoInformatique;

impl Site for CarthagoInformatique {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}