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
    name: "SBSInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product h2").unwrap()),
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
        (Section::GamingPC, "https://www.sbsinformatique.com/pc-gamer-tunisie"),
        (Section::GamingPC, "https://www.sbsinformatique.com/stations-pro-tunisie"),
        (Section::PC, "https://www.sbsinformatique.com/pcs-de-bureau-tunisie"),
        (Section::GamingLaptop, "https://www.sbsinformatique.com/pc-portable-tunisie"),
        (Section::Monitor, "https://www.sbsinformatique.com/moniteurs-tunisie"),
        (Section::CPU, "https://www.sbsinformatique.com/processeur-tunisie"),
        (Section::GPU, "https://www.sbsinformatique.com/cartes-graphiques-tunisie"),
        (Section::RAM, "https://www.sbsinformatique.com/barrettes-memoires-tunisie"),
        (Section::MotherBoard, "https://www.sbsinformatique.com/carte-mere-tunisie"),
        (Section::Storage, "https://www.sbsinformatique.com/stockage-hdd-ssd-tunisie"),
        (Section::Case, "https://www.sbsinformatique.com/boitiers-pc-tunisie"),
        (Section::PSU, "https://www.sbsinformatique.com/alimentations-tunisie"),
        (Section::Cooler, "https://www.sbsinformatique.com/refroidissement-boitier-tunisie"),
        (Section::Cooler, "https://www.sbsinformatique.com/refroidissement-cpu-tunisie"),
    ]
};

pub struct SBSInformatique;

impl Site for SBSInformatique {
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
            .replace("produits", "")
            .replace("produit", "")
            .trim().to_string();

        let count = text.parse::<i32>().map_err(|err| format!("text is `{text}`: {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}