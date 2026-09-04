use crate::core::product::ProductStatus;
use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::core::tracking::scrape_error::ProductParseError;
use crate::utils::scraper_ext::ElementRefExt;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use std::str::FromStr;

static CONFIG: RetailerConfig = RetailerConfig {
    name: "TechSpace",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.woocommerce-pagination ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("ul.products li.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h3.woocommerce-loop-product__title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.product-image img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price span bdi").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.price del span bdi").unwrap()),
    price_sel_2: Some(Lazy::new(|| Selector::parse("span.price ins span bdi").unwrap())),
    status_sel: Some(Lazy::new(|| Selector::parse("span.inventory_status").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-product-details__short-description").unwrap())),
    empty_page_sel: Some(Lazy::new(|| Selector::parse("div.woocommerce-no-products-found").unwrap())),
    sections: &[
        (Section::GamingPC, "https://techspace.tn/pc-gamer-tunisie/"),
        (Section::GamingPC, "https://techspace.tn/full-setup/"),
        (Section::Laptop, "https://techspace.tn/pc-portable-tunisie/"),
        (Section::Monitor, "https://techspace.tn/ecrans-gaming/"),
        (Section::CPU, "https://techspace.tn/processeur-intel/"),
        (Section::CPU, "https://techspace.tn/processeur-amd/"),
        (Section::GPU, "https://techspace.tn/carte-graphique/"),
        (Section::Memory, "https://techspace.tn/barette-memoire/"),
        (Section::Storage, "https://techspace.tn/stockage/"),
        (Section::Motherboard, "https://techspace.tn/carte-mere-intel/"),
        (Section::Motherboard, "https://techspace.tn/carte-mere-amd/"),
        (Section::Cooler, "https://techspace.tn/air-cooling/"),
        (Section::Cooler, "https://techspace.tn/water-cooling/"),
        (Section::Cooler, "https://techspace.tn/ventilateur/"),
        (Section::PowerSupply, "https://techspace.tn/boite-dalimentation/"),
        (Section::Case, "https://techspace.tn/boitier/"),
        (Section::Mouse, "https://techspace.tn/souris/"),
        (Section::Keyboard, "https://techspace.tn/clavier/"),
        (Section::MousePad, "https://techspace.tn/tapis/"),
        (Section::Headphones, "https://techspace.tn/casque/"),
        (Section::AccessoriesCombo, "https://techspace.tn/combo/"),
        (Section::Controller, "https://techspace.tn/manette/"),
        (Section::ConsoleAccessories, "https://techspace.tn/volant/"),
    ],
};

pub struct TechSpace;

impl Retailer for TechSpace {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, ProductParseError> {
        match &self.config().status_sel {
            Some(sel) => {
                let status = element.select_text(sel, "status")?.replace("Availability:", "");
                ProductStatus::from_str(status.trim()).map_err(|_| ProductParseError::UnknownStatus { value: status })
            }
            None => Ok(ProductStatus::InStock),
        }
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}
