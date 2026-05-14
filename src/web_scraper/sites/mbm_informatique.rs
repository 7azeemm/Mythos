use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::ProductStatus;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::ElementRefExt;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

const PRODUCTS_PER_PAGE: i32 = 12;

static CONFIG: SiteConfig = SiteConfig {
    name: "MBMInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product p").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article.item").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("div.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.tvproduct-cart-btn form button").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.tv-product-desc").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::Laptop, "https://mbm-tn.com/145-pc-portable"),
        (Section::Laptop, "https://mbm-tn.com/257-macbook"),
        (Section::GamingLaptop, "https://mbm-tn.com/256-pc-portable-gamer"),
        (Section::AllInOnePC, "https://mbm-tn.com/232-pc-tout-en-un-all-in-one"),
        (Section::PC, "https://mbm-tn.com/76-pc-de-bureau"),
        (Section::PC, "https://mbm-tn.com/111-mac"),
        (Section::GamingPC, "https://mbm-tn.com/325-pc-de-bureau-gamer"),
        (Section::Monitor, "https://mbm-tn.com/56-ecran"),
        (Section::Monitor, "https://mbm-tn.com/291-ecran-gamer"),
        (Section::CPU, "https://mbm-tn.com/124-processeur"),
        (Section::GPU, "https://mbm-tn.com/86-cartes-graphique"),
        (Section::RAM, "https://mbm-tn.com/85-barrettes-memoire-dimm"),
        (Section::RAM, "https://mbm-tn.com/110-barrettes-memoire-so-dimm"),
        (Section::MotherBoard, "https://mbm-tn.com/109-cartes-meres"),
        (Section::MotherBoard, "https://mbm-tn.com/87-cartes-meres"),
        (Section::Storage, "https://mbm-tn.com/174-disque-ssd"),
        (Section::Storage, "https://mbm-tn.com/176-boitier-disque-dur"),
        (Section::Storage, "https://mbm-tn.com/334-disque-dur-interne"),
        (Section::Storage, "https://mbm-tn.com/66-disques-dur-internes"),
        (Section::Storage, "https://mbm-tn.com/59-disque-dur-interne"),
        (Section::Case, "https://mbm-tn.com/335-boitier"),
        (Section::Cooler, "https://mbm-tn.com/71-ventilateurs"),
        (Section::Cooler, "https://mbm-tn.com/155-ventilateurs"),
        (Section::Cooler, "https://mbm-tn.com/143-refroidisseur"),
        (Section::PSU, "https://mbm-tn.com/108-blocs-alimentation-"),
    ]
};

pub struct MBMInformatique;

impl Site for MBMInformatique {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, String> {
        Ok(match element.select_elem(&self.config().status_sel.as_ref().unwrap(), "status")?.attr("disabled") {
            Some(_) => ProductStatus::OutOfStock,
            None => ProductStatus::InStock
        })
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let element = doc.select(&self.config().nav_sel)
            .next()
            .ok_or("products count not found")?
            .get_text();

        let text = element
            .replace("products", "")
            .replace("product", "")
            .trim().to_string();
        
        let count = text.parse::<i32>().map_err(|err| format!("text is `{text}`: {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}