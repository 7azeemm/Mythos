use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::ProductStatus;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::ElementRefExt;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

const PRODUCTS_PER_PAGE: i32 = 3;

static CONFIG: SiteConfig = SiteConfig {
    name: "Scoop",
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
        (Section::Laptop, "https://www.scoop.com.tn/321-ordinateurs-portables"),
        (Section::GamingLaptop, "https://www.scoop.com.tn/192-pc-portable-gamer"),
        (Section::PC, "https://www.scoop.com.tn/291-pc-de-bureau"),
        (Section::GamingPC, "https://www.scoop.com.tn/2185-gamme-alpha-by-scoop"),
        (Section::GamingPC, "https://www.scoop.com.tn/198-pc-gamer-brande"),
        (Section::GamingPC, "https://www.scoop.com.tn/2128-powered-by-msi"),
        (Section::AllInOnePC, "https://www.scoop.com.tn/127-pc-tout-en-un"),
        (Section::Monitor, "https://www.scoop.com.tn/209-ecrans-gaming"),
        (Section::Monitor, "https://www.scoop.com.tn/208-ecrans-professionnels"),
        (Section::CPU, "https://www.scoop.com.tn/253-processeur"),
        (Section::GPU, "https://www.scoop.com.tn/168-carte-graphique"),
        (Section::RAM, "https://www.scoop.com.tn/139-memoire-pc"),
        (Section::MotherBoard, "https://www.scoop.com.tn/170-carte-mere"),
        (Section::Storage, "https://www.scoop.com.tn/261-disque-ssd"),
        (Section::Storage, "https://www.scoop.com.tn/2131-disque-nvme"),
        (Section::Storage, "https://www.scoop.com.tn/2132-disque-hdd"),
        (Section::Case, "https://www.scoop.com.tn/169-boitier"),
        (Section::PSU, "https://www.scoop.com.tn/179-alimentation"),
        (Section::Cooler, "https://www.scoop.com.tn/180-refroidissement"),
        (Section::Cooler, "https://www.scoop.com.tn/272-ventilateur"),
    ]
};

pub struct Scoop;

impl Site for Scoop {
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
        
        let count = text.parse::<i32>()
            .map_err(|err| format!("text is `{text}`: {err}"))?;

        Ok((count + PRODUCTS_PER_PAGE - 1) / PRODUCTS_PER_PAGE)
    }
}