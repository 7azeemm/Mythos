use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::ProductStatus;
use crate::web_scraper::sections::Section;
use crate::web_scraper::utils::ElementRefExt;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};

static CONFIG: SiteConfig = SiteConfig {
    name: "Jumbo",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("div.product_name a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("button.add-to-cart i").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.decriptions-short").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::PC, "https://jumbo.tn/61-pc-de-bureau"),
        (Section::GamingPC, "https://jumbo.tn/141-pc-de-bureau-gamer"),
        (Section::GamingPC, "https://jumbo.tn/615-ordinateur-de-bureau-gamer"),
        (Section::AllInOnePC, "https://jumbo.tn/548-pc-tout-en-un"),
        (Section::Laptop, "https://jumbo.tn/57-pc-portable"),
        (Section::Laptop, "https://jumbo.tn/574-pc-portable-pro"),
        (Section::GamingLaptop, "https://jumbo.tn/140-pc-portable-gamer"),
        (Section::GamingLaptop, "https://jumbo.tn/614-pc-portable-gamer"),
        (Section::MacBook, "https://jumbo.tn/541-macbook"),
        (Section::Monitor, "https://jumbo.tn/654-ecran-pro-lfd"),
        (Section::Monitor, "https://jumbo.tn/544-ecran"),
        (Section::Monitor, "https://jumbo.tn/142-ecran-pc-gamer"),
        (Section::Mouse, "https://jumbo.tn/454-souris-gamer"),
        (Section::Keyboard, "https://jumbo.tn/455-clavier-gamer"),
        (Section::Keyboard, "https://jumbo.tn/158-clavier"),
        (Section::AccessoriesCombo, "https://jumbo.tn/564-clavier-souris"),
        (Section::CPU, "https://jumbo.tn/637-processeur"),
        (Section::GPU, "https://jumbo.tn/638-carte-graphique"),
        (Section::Memory, "https://jumbo.tn/640-barrette-memoire"),
        (Section::Motherboard, "https://jumbo.tn/639-carte-mere"),
        (Section::Storage, "https://jumbo.tn/144-disque-dur-interne"),
        (Section::Storage, "https://jumbo.tn/556-disque-dur-ssd"),
        (Section::PowerSupply, "https://jumbo.tn/643-bloc-d-alimentation"),
        (Section::Case, "https://jumbo.tn/641-boitier"),
        (Section::Case, "https://jumbo.tn/485-boitier-pc-gamer"),
        (Section::Cooler, "https://jumbo.tn/642-ventilateur"),
        (Section::Cooler, "https://jumbo.tn/581-refroidisseur"),
    ]
};

pub struct Jumbo;

impl Site for Jumbo {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, String> {
        Ok(element.select_elem(&CONFIG.status_sel.as_ref().unwrap(), "status")?
            .attr("fa-ban")
            .map(|_| ProductStatus::OutOfStock)
            .unwrap_or(ProductStatus::InStock))
    }
}