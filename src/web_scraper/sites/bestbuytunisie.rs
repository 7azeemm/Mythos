use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: SiteConfig = SiteConfig {
    name: "BestBuyTunisie",
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
    empty_page_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-no-products-found").unwrap())),
    sections: &[
        (Section::PC, "https://bestbuytunisie.tn/vente/informatique/pc/pc-de-bureau-tunisie/"),
        (Section::GamingPC, "https://bestbuytunisie.tn/vente/gaming/pc-gamer-tunisie/"),
        (Section::AllInOnePC, "https://bestbuytunisie.tn/vente/informatique/pc/pc-tout-en-un-tunisie/"),
        (Section::AllInOnePC, "https://bestbuytunisie.tn/vente/informatique/ordinateur-de-bureau/imac-tunisie/"),
        (Section::Laptop, "https://bestbuytunisie.tn/vente/informatique/pc/pc-portable-tunisie/"),
        (Section::GamingLaptop, "https://bestbuytunisie.tn/vente/informatique/pc/pc-portable-gamer-tunisie/"),
        (Section::MacBook, "https://bestbuytunisie.tn/vente/informatique/pc/mac-tunisie/"),
        (Section::Monitor, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/ecran-tunisie/"),
        (Section::Monitor, "https://bestbuytunisie.tn/vente/gaming/peripheriques-et-accessoires-gamers/ecrans-gamer-tunisie/"),
        (Section::CPU, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/processeur-tunisie/"),
        (Section::GPU, "https://bestbuytunisie.tn/vente/gaming/composants/carte-graphique-tunisie/"),
        (Section::Memory, "https://bestbuytunisie.tn/vente/gaming/composants/barrette-memoire-tunisie/"),
        (Section::Storage, "https://bestbuytunisie.tn/vente/informatique/stockage/disque-dur-interne-tunisie/"),
        (Section::Storage, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/disque-dur-ssd-hdd-mvme-tunisie/"),
        (Section::Motherboard, "https://bestbuytunisie.tn/vente/informatique/composants-pc/carte-mere-pc-tunisie/"),
        (Section::Cooler, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/refroidissement-tunisie/"),
        (Section::PowerSupply, "https://bestbuytunisie.tn/vente/informatique/composants-pc/bloc-dalimentation-pc-tunisie/"),
        (Section::PowerSupply, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/bloc-dalimentation-tunisie/"),
        (Section::Case, "https://bestbuytunisie.tn/vente/gaming/composant-pc-gamer/boitier-pc-gamer-tunisie/"),
        (Section::Mouse, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/souris-tunisie/"),
        (Section::Mouse, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/souris-gaming-tunisie/"),
        (Section::Keyboard, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/claviers-tunisie/"),
        (Section::Keyboard, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/clavier-gaming-tunisie/"),
        (Section::MousePad, "https://bestbuytunisie.tn/vente/gaming/peripheriques-et-accessoires-gamers/tapis-de-souris-gamer-tunisie/"),
        (Section::MousePad, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/tapis-de-souris-tunisie/"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/accessoires-telephonie/ecouteurs-et-kit-pieton-tunisie/"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/airpuds-tunisie/"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/micro-casques-tunisie/"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/multimedia/son-numerique/radio-reveil-station-meteo/casque-tunisie/"),
        (Section::Headphones, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/casque-gaming-tunisie/"),
        (Section::GamingChair, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/chaise-gaming-tunisie/"),
        (Section::AccessoriesCombo, "https://bestbuytunisie.tn/vente/informatique/accessoires-ordinateur/ensemble-clavier-souris-tunisie/"),
        (Section::Console, "https://bestbuytunisie.tn/vente/gaming/console-de-jeux/ps5-tunisie/"),
        (Section::Console, "https://bestbuytunisie.tn/vente/gaming/console-de-jeux/xbox-1-tunisie/"),
        (Section::Console, "https://bestbuytunisie.tn/vente/gaming/console-de-jeux/nintendo-switch-tunisie/"),
        (Section::Console, "https://bestbuytunisie.tn/vente/gaming/console-de-jeux/ps4-tunisie/"),
        (Section::Controller, "https://bestbuytunisie.tn/vente/gaming/accessoires-gaming/manette-de-jeu-tunisie/"),
        (Section::Controller, "https://bestbuytunisie.tn/vente/gaming/accessoires-consoles/manettes-tunisie/"),
        (Section::ConsoleGame, "https://bestbuytunisie.tn/vente/gaming/accessoires-consoles/jeux-video-tunisie/"),
        (Section::ConsoleAccessories, "https://bestbuytunisie.tn/vente/gaming/accessoires-consoles/accessoires-console-divers-tunisie/"),
        (Section::ConsoleAccessories, "https://bestbuytunisie.tn/vente/gaming/accessoires-consoles/casque-de-realite-virtuelle-tunisie/"),
        (Section::Smartphone, "https://bestbuytunisie.tn/vente/smartphones-smartphone-mobile-tunisie/"),
        (Section::Smartphone, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/smartphone-mobile/iphone-tunisie/"),
        (Section::Tablet, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/tablettes/tablettes-android-tunisie/"),
        (Section::Tablet, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/tablettes/ipad-tunisie/"),
        (Section::Smartwatch, "https://bestbuytunisie.tn/vente/telephonie-et-tablette/smartwatch/montre-connectee-tunisie/"),
        (Section::Television, "https://bestbuytunisie.tn/vente/electromenager/gros-electromenager/televiseur-tunisie/"),
    ]
};

pub struct BestBuyTunisie;

impl Site for BestBuyTunisie {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}