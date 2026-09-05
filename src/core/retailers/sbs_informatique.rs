use crate::core::product::ProductStatus;
use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::core::tracking::scrape_error::{PaginationError, ProductParseError};
use crate::utils::scraper_ext::ElementRefExt;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

const PRODUCTS_PER_PAGE: i32 = 12;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "SBSInformatique",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("div#js-product-list-top div.tv-total-product h2").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article.item").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("div.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    original_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.tvproduct-cart-btn form button").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("div.tv-product-desc").unwrap())),
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.tvproduct-page-decs").unwrap())),
    empty_page_sel: Some(Lazy::new(|| Selector::parse("section.page-not-found").unwrap())),
    sections: &[
        (Section::PC, "https://www.sbsinformatique.com/pcs-de-bureau-tunisie"),
        (Section::GamingPC, "https://www.sbsinformatique.com/pc-gamer-tunisie"),
        (Section::GamingPC, "https://www.sbsinformatique.com/stations-pro-tunisie"),
        (Section::Laptop, "https://www.sbsinformatique.com/pc-portable-tunisie"),
        (Section::Monitor, "https://www.sbsinformatique.com/moniteurs-tunisie"),
        (Section::CPU, "https://www.sbsinformatique.com/processeur-tunisie"),
        (Section::GPU, "https://www.sbsinformatique.com/cartes-graphiques-tunisie"),
        (Section::Memory, "https://www.sbsinformatique.com/barrettes-memoires-tunisie"),
        (Section::Storage, "https://www.sbsinformatique.com/stockage-hdd-ssd-tunisie"),
        (Section::Motherboard, "https://www.sbsinformatique.com/carte-mere-tunisie"),
        (Section::Cooler, "https://www.sbsinformatique.com/refroidissement-boitier-tunisie"),
        (Section::Cooler, "https://www.sbsinformatique.com/refroidissement-cpu-tunisie"),
        (Section::PowerSupply, "https://www.sbsinformatique.com/alimentations-tunisie"),
        (Section::Case, "https://www.sbsinformatique.com/boitiers-pc-tunisie"),
        (Section::Mouse, "https://www.sbsinformatique.com/souris-gamer-tunisie"),
        (Section::Keyboard, "https://www.sbsinformatique.com/claviers-gamer-tunisie"),
        (Section::MousePad, "https://www.sbsinformatique.com/tapis-gamer-tunisie"),
        (Section::Headphones, "https://www.sbsinformatique.com/casques-gamer-tunisie"),
        (Section::Headphones, "https://www.sbsinformatique.com/casque-ecouteur-tunisie"),
        (Section::GamingChair, "https://www.sbsinformatique.com/chaise-gaming-tunisie"),
        (Section::AccessoriesCombo, "https://www.sbsinformatique.com/packs-gaming-tunisie"),
        (Section::Controller, "https://www.sbsinformatique.com/manettes-volants-tunisie"),
        (Section::Smartphone, "https://www.sbsinformatique.com/smartphone-tunisie"),
        (Section::Tablet, "https://www.sbsinformatique.com/tablette-tunisie"),
        (Section::Smartwatch, "https://www.sbsinformatique.com/montre-connectee-tunisie"),
        (Section::Television, "https://www.sbsinformatique.com/televiseurs-tunisie"),
    ],
};

pub struct SBSInformatique;

impl Retailer for SBSInformatique {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, ProductParseError> {
        Ok(match element.select_elem(&self.config().status_sel.as_ref().unwrap(), "status")?.attr("disabled") {
            Some(_) => ProductStatus::OutOfStock,
            None => ProductStatus::InStock,
        })
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, PaginationError> {
        let element = doc.select(&self.config().nav_sel).next().ok_or(PaginationError::MissingValue)?.get_text();

        let text = element.replace("produits", "").replace("produit", "").trim().to_string();

        let count = text.parse::<i32>().map_err(|_| PaginationError::InvalidValue { value: text })?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}
