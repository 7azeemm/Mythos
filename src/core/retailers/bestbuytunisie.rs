use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "BestBuyTunisie",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("div.items-center button").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.w-full div.bg-white").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("div.p-4 a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.block img[alt]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.font-extrabold").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.line-through").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("span.text-[10px]").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.prose").unwrap())),
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://bestbuytunisie.tn/vente/informatique/ordinateur-de-bureau/pc-de-bureau-tunisie"),
        (Section::AllInOnePC, "https://bestbuytunisie.tn/vente/informatique/ordinateur-de-bureau/pc-tout-en-un-tunisie"),
        (Section::AllInOnePC, "https://bestbuytunisie.tn/vente/imac-tunisie"),
        (Section::Laptop, "https://bestbuytunisie.tn/vente/pc-portable-tunisie"),
        (Section::MacBook, "https://bestbuytunisie.tn/vente/mac-tunisie"),
        (Section::CPU, "https://bestbuytunisie.tn/vente/processeur-tunisie"),
        (Section::GPU, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/carte-graphique-tunisie"),
        (Section::Memory, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/barrette-memoire-tunisie"),
        (Section::Storage, "https://bestbuytunisie.tn/vente/disque-dur-interne-tunisie"),
        (Section::Storage, "https://bestbuytunisie.tn/vente/disque-dur-ssd-tunisie"),
        (Section::Storage, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/disque-dur-ssdhddmvme-tunisie"),
        (Section::Motherboard, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/carte-mere-pc-tunisie"),
        (Section::Cooler, "https://bestbuytunisie.tn/vente/refroidissement-tunisie"),
        (Section::PowerSupply, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/bloc-dalimentation-tunisie"),
        (Section::Case, "https://bestbuytunisie.tn/vente/boitier-pc-gamer-tunisie"),
        (Section::Mouse, "https://bestbuytunisie.tn/vente/souris-tunisie"),
        (Section::Mouse, "https://bestbuytunisie.tn/vente/souris-gaming-tunisie"),
        (Section::Keyboard, "https://bestbuytunisie.tn/vente/clavier-gaming-tunisie"),
        (Section::Keyboard, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/clavier-tunisie"),
        (Section::MousePad, "https://bestbuytunisie.tn/vente/tapis-de-souris-tunisie"),
        (Section::MousePad, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/tapis-de-souris-gamer-tunisie"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/micro-casques-tunisie"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/airpuds-tunisie"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/casque-gaming-tunisie"),
        (Section::GamingChair, "https://bestbuytunisie.tn/vente/chaise-gaming-tunisie"),
        (Section::AccessoriesCombo, "https://bestbuytunisie.tn/vente/ensemble-clavier-souris-tunisie"),
        (Section::Monitor, "https://bestbuytunisie.tn/vente/ecran-tunisie"),
        (Section::Console, "https://bestbuytunisie.tn/vente/nintendo-switch-tunisie"),
        (Section::Console, "https://bestbuytunisie.tn/vente/ps4-tunisie"),
        (Section::Console, "https://bestbuytunisie.tn/vente/ps5-tunisie"),
        (Section::ConsoleGame, "https://bestbuytunisie.tn/vente/jeux-video-tunisie"),
        (Section::Controller, "https://bestbuytunisie.tn/vente/manette-de-jeu-tunisie"),
        (Section::Smartphone, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/smartphone-mobile/smartphones-tunisie"),
        (Section::Smartphone, "https://bestbuytunisie.tn/vente/iphone-tunisie"),
        (Section::Tablet, "https://bestbuytunisie.tn/vente/tablettes-android-tunisie"),
        (Section::Tablet, "https://bestbuytunisie.tn/vente/ipad-tunisie"),
        (Section::Smartwatch, "https://bestbuytunisie.tn/vente/montre-connectee-tunisie"),
        (Section::Television, "https://bestbuytunisie.tn/vente/televiseur-tunisie"),
    ],
};

pub struct BestBuyTunisie;

impl Retailer for BestBuyTunisie {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}
