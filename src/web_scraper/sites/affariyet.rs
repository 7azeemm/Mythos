use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "Affariyet",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.product-list div[data-id-product] article").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.product-thumbnail a.product-cover-link img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: None,
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-description-short").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::PC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/pc-de-bureau/"),
        (Section::PC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/imac/"),
        (Section::GamingPC, "https://www.affariyet.com/gaming/pc-gamer/ordinateur-gamer-/"),
        (Section::AllInOnePC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/tout-en-un-/"),
        (Section::Laptop, "https://www.affariyet.com/informatique/ordinateurs-portables/"),
        (Section::GamingLaptop, "https://www.affariyet.com/gaming/pc-gamer/pc-portable-gamer/"),
        (Section::Monitor, "https://www.affariyet.com/informatique/ordinateur-de-bureau/ecran/"),
        (Section::Monitor, "https://www.affariyet.com/gaming/pc-gamer/ecran-gamer/"),
        (Section::CPU, "https://www.affariyet.com/informatique/composants-informatique-/processeur/"),
        (Section::GPU, "https://www.affariyet.com/informatique/composants-informatique-/carte-graphique/"),
        (Section::GPU, "https://www.affariyet.com/gaming/composants-gamer/carte-graphique-/"),
        (Section::RAM, "https://www.affariyet.com/informatique/composants-informatique-/barrettes-memoire/"),
        (Section::RAM, "https://www.affariyet.com/gaming/composants-gamer/barrette-memoire-gamer/"),
        (Section::MotherBoard, "https://www.affariyet.com/informatique/composants-informatique-/carte-mere/"),
        (Section::Storage, "https://www.affariyet.com/informatique/stockage-/disques-durs-internes/"),
        (Section::PSU, "https://www.affariyet.com/informatique/composants-informatique-/bloc-d-alimentation/"),
        (Section::PSU, "https://www.affariyet.com/gaming/composants-gamer/alimentation-pc-gamer/"),
        (Section::Cooler, "https://www.affariyet.com/gaming/composants-gamer/ventilateur-refroidisseur-pc/"),
        (Section::Case, "https://www.affariyet.com/gaming/composants-gamer/boitier-gaming/"),
    ]
};

pub struct Affariyet;

impl Site for Affariyet {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}