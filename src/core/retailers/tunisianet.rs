use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::Selector;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "Tunisianet",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div.item-product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div#stock_availability").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse(r#"div.product-description div[itemprop="description"]"#).unwrap())),
    page_desc_sel: None,
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://www.tunisianet.com.tn/373-pc-de-bureau"),
        (Section::GamingPC, "https://www.tunisianet.com.tn/682-pc-de-bureau-gamer"),
        (Section::GamingPC, "https://www.tunisianet.com.tn/732-full-setup-gamer"),
        (Section::AllInOnePC, "https://www.tunisianet.com.tn/686-pc-tout-en-un"),
        (Section::Laptop, "https://www.tunisianet.com.tn/301-pc-portable-tunisie"),
        (Section::Laptop, "https://www.tunisianet.com.tn/703-pc-portable-pro"),
        (Section::GamingLaptop, "https://www.tunisianet.com.tn/681-pc-portable-gamer"),
        (Section::Monitor, "https://www.tunisianet.com.tn/667-ecran-pc-tunisie"),
        (Section::CPU, "https://www.tunisianet.com.tn/421-processeur"),
        (Section::GPU, "https://www.tunisianet.com.tn/410-carte-graphique-tunisie"),
        (Section::Memory, "https://www.tunisianet.com.tn/409-barrette-memoire"),
        (Section::Storage, "https://www.tunisianet.com.tn/408-disque-dur-interne"),
        (Section::Storage, "https://www.tunisianet.com.tn/379-disques-ssd"),
        (Section::Motherboard, "https://www.tunisianet.com.tn/420-carte-mere"),
        (Section::Cooler, "https://www.tunisianet.com.tn/427-refroidisseur-ventilateur-boitier"),
        (Section::PowerSupply, "https://www.tunisianet.com.tn/423-boite-alimentation-pc-tunisie"),
        (Section::Case, "https://www.tunisianet.com.tn/425-boitier"),
        (Section::Mouse, "https://www.tunisianet.com.tn/334-souris-informatique"),
        (Section::Keyboard, "https://www.tunisianet.com.tn/704-claviers"),
        (Section::MousePad, "https://www.tunisianet.com.tn/488-tapis-souris-tunisie"),
        (Section::Headphones, "https://www.tunisianet.com.tn/338-casque-ecouteurs"),
        (Section::Console, "https://www.tunisianet.com.tn/467-consoles"),
        (Section::Controller, "https://www.tunisianet.com.tn/341-manettes-de-jeux"),
        (Section::Smartphone, "https://www.tunisianet.com.tn/596-smartphone-tunisie"),
        (Section::Tablet, "https://www.tunisianet.com.tn/515-tablette"),
        (Section::Smartwatch, "https://www.tunisianet.com.tn/650-smartwatch"),
        (Section::Television, "https://www.tunisianet.com.tn/665-televiseurs"),
    ],
};

pub struct Tunisianet;

impl Retailer for Tunisianet {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }
}
