use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::ProductStatus;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::ElementRefExt;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use std::str::FromStr;

static CONFIG: SiteConfig = SiteConfig {
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
    sections: &[
        (Section::Laptop, "https://techspace.tn/pc-portable-tunisie/"),
        (Section::GamingPC, "https://techspace.tn/pc-gamer-tunisie/"),
        (Section::GamingSetup, "https://techspace.tn/full-setup/"),
        (Section::Monitor, "https://techspace.tn/ecrans-gaming/"),
        (Section::CPU, "https://techspace.tn/processeur-intel/"),
        (Section::CPU, "https://techspace.tn/processeur-amd/"),
        (Section::GPU, "https://techspace.tn/carte-graphique/"),
        (Section::RAM, "https://techspace.tn/barette-memoire/"),
        (Section::MotherBoard, "https://techspace.tn/carte-mere-intel/"),
        (Section::MotherBoard, "https://techspace.tn/carte-mere-amd/"),
        (Section::Storage, "https://techspace.tn/stockage/"),
        (Section::PSU, "https://techspace.tn/boite-dalimentation/"),
        (Section::Cooler, "https://techspace.tn/air-cooling/"),
        (Section::Cooler, "https://techspace.tn/water-cooling/"),
        (Section::Cooler, "https://techspace.tn/ventilateur/"),
        (Section::Case, "https://techspace.tn/boitier/"),
    ]
};

pub struct TechSpace;

impl Site for TechSpace {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }
    
    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, String> {
        match &self.config().status_sel {
            Some(sel) => {
                let status = element.select_text(sel, "status")?.replace("Availability:", "");
                Ok(ProductStatus::from_str(status.trim())?)
            },
            None => Ok(ProductStatus::InStock)
        }
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}page/{page}/")
    }
}