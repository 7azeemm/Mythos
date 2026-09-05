use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "GamerShop",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product_name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img.product_image").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-quantities label").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.product-des").unwrap())),
    page_desc_sel: None,
    empty_page_sel: None,
    sections: &[
        (Section::GamingPC, "https://gamershop.tn/203-pc-gamer-tunisie"),
        (Section::GamingLaptop, "https://gamershop.tn/204-pc-portable-gamer"),
        (Section::Monitor, "https://gamershop.tn/913-ecran"),
        (Section::CPU, "https://gamershop.tn/399-processeur-intel"),
        (Section::CPU, "https://gamershop.tn/910-processeur-amd"),
        (Section::GPU, "https://gamershop.tn/397-carte-graphique"),
        (Section::Memory, "https://gamershop.tn/398-barrette-memoire"),
        (Section::Storage, "https://gamershop.tn/395-disque-dur-ssd"),
        (Section::Motherboard, "https://gamershop.tn/394-carte-mere-intel"),
        (Section::Motherboard, "https://gamershop.tn/911-carte-mere-amd"),
        (Section::Cooler, "https://gamershop.tn/936-watercooling"),
        (Section::Cooler, "https://gamershop.tn/937-aircooling"),
        (Section::PowerSupply, "https://gamershop.tn/724-bloc-d-alimentation"),
        (Section::Case, "https://gamershop.tn/393-boitier"),
        (Section::Mouse, "https://gamershop.tn/219-souris-gamer"),
        (Section::Keyboard, "https://gamershop.tn/221-clavier-gamer"),
        (Section::MousePad, "https://gamershop.tn/220-tapis-de-souris-gamer"),
        (Section::Headphones, "https://gamershop.tn/666-airpods"),
        (Section::Headphones, "https://gamershop.tn/328-casque-ecouteurs"),
        (Section::Headphones, "https://gamershop.tn/259-casque-gaming"),
        (Section::GamingChair, "https://gamershop.tn/964-msi-chaise-gaming"),
        (Section::GamingChair, "https://gamershop.tn/965-trust-chaise-gaming"),
        (Section::AccessoriesCombo, "https://gamershop.tn/743-pack-ensemble"),
        (Section::Console, "https://gamershop.tn/916-console"),
        (Section::Controller, "https://gamershop.tn/409-manette"),
        (Section::ConsoleGame, "https://gamershop.tn/390-jeux-video-tunisie"),
        (Section::ConsoleAccessories, "https://gamershop.tn/391-accessoires-console-tunisie"),
        (Section::ConsoleAccessories, "https://gamershop.tn/941-volant-pc-gamer"),
        (Section::Smartphone, "https://gamershop.tn/13-smartphone-mobile-tunisie"),
        (Section::Smartwatch, "https://gamershop.tn/148-smartwatch-tunisie"),
        (Section::Television, "https://gamershop.tn/217-tv"),
    ],
};

pub struct GamerShop;

impl Retailer for GamerShop {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }
}
