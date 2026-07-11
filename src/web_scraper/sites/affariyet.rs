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
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/pc-de-bureau/"),
        (Section::GamingPC, "https://www.affariyet.com/gaming/pc-gamer/ordinateur-gamer-/"),
        (Section::AllInOnePC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/tout-en-un-/"),
        (Section::AllInOnePC, "https://www.affariyet.com/informatique/ordinateur-de-bureau/imac/"),
        (Section::Laptop, "https://www.affariyet.com/informatique/ordinateurs-portables/pc-portable/"),
        (Section::GamingLaptop, "https://www.affariyet.com/gaming/pc-gamer/pc-portable-gamer/"),
        (Section::MacBook, "https://www.affariyet.com/informatique/ordinateurs-portables/mac/"),
        (Section::Monitor, "https://www.affariyet.com/informatique/ordinateur-de-bureau/ecran/"),
        (Section::Monitor, "https://www.affariyet.com/gaming/pc-gamer/ecran-gamer/"),
        (Section::CPU, "https://www.affariyet.com/informatique/composants-informatique-/processeur/"),
        (Section::GPU, "https://www.affariyet.com/informatique/composants-informatique-/carte-graphique/"),
        (Section::GPU, "https://www.affariyet.com/gaming/composants-gamer/carte-graphique-/"),
        (Section::Memory, "https://www.affariyet.com/informatique/composants-informatique-/barrettes-memoire/"),
        (Section::Memory, "https://www.affariyet.com/gaming/composants-gamer/barrette-memoire-gamer/"),
        (Section::Memory, "https://www.affariyet.com/informatique/composant-de-serveur/barette-memoire-pour-serveur/"),
        (Section::Storage, "https://www.affariyet.com/informatique/stockage-/disques-durs-internes/"),
        (Section::Motherboard, "https://www.affariyet.com/informatique/composants-informatique-/carte-mere/"),
        (Section::Cooler, "https://www.affariyet.com/gaming/composants-gamer/ventilateur-refroidisseur-pc/"),
        (Section::PowerSupply, "https://www.affariyet.com/informatique/composants-informatique-/bloc-d-alimentation/"),
        (Section::PowerSupply, "https://www.affariyet.com/gaming/composants-gamer/alimentation-pc-gamer/"),
        (Section::Case, "https://www.affariyet.com/gaming/composants-gamer/boitier-gaming/"),
        (Section::Mouse, "https://www.affariyet.com/gaming/accessoires-gamer/souris-gamer/"),
        (Section::Keyboard, "https://www.affariyet.com/gaming/accessoires-gamer/clavier-gaming/"),
        (Section::MousePad, "https://www.affariyet.com/gaming/accessoires-gamer/tapis-de-souris-gamer-/"),
        (Section::Headphones, "https://www.affariyet.com/tv-photo-son/son-numeriques/casques-micro/"),
        (Section::Headphones, "https://www.affariyet.com/tv-photo-son/son-numeriques/earbuds/"),
        (Section::Headphones, "https://www.affariyet.com/gaming/accessoires-gamer/micro-casque-gamer/"),
        (Section::GamingChair, "https://www.affariyet.com/gaming/streaming-/chaise-gaming/"),
        (Section::AccessoriesCombo, "https://www.affariyet.com/informatique/peripherique-et-accessoires/clavier-souris/"),
        (Section::Console, "https://www.affariyet.com/gaming/console-de-jeux-/playstation/"),
        (Section::Console, "https://www.affariyet.com/gaming/console-de-jeux-/nintendo/"),
        (Section::Controller, "https://www.affariyet.com/gaming/accessoires-de-jeux/manettes/"),
        (Section::ConsoleGame, "https://www.affariyet.com/gaming/accessoires-de-jeux/jeux-video-/"),
        (Section::ConsoleAccessories, "https://www.affariyet.com/gaming/accessoires-de-jeux/accessoires-jeux-de-course/"),
        (Section::Smartphone, "https://www.affariyet.com/telephonie/iphone/"),
        (Section::Smartphone, "https://www.affariyet.com/telephonie/smartphone-tunisie/"),
        (Section::Tablet, "https://www.affariyet.com/informatique/38-tablettes-tactiles/ipad/"),
        (Section::Tablet, "https://www.affariyet.com/informatique/38-tablettes-tactiles/tablettes-android/"),
        (Section::Smartwatch, "https://www.affariyet.com/telephonie/smart-watch/"),
        (Section::Television, "https://www.affariyet.com/tv-photo-son/televiseurs/tv-led/")
    ]
};

pub struct Affariyet;

impl Site for Affariyet {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
}