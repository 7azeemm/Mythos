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
    name: "ScoopGaming",
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
        (Section::GamingLaptop, "https://www.scoopgaming.com.tn/62-pc-portable-gamer"),
        (Section::GamingPC, "https://www.scoopgaming.com.tn/56-pc-gamer"),
        (Section::Monitor, "https://www.scoopgaming.com.tn/58-ecrans-gaming"),
        (Section::Monitor, "https://www.scoopgaming.com.tn/61-ecrans-professionnels"),
        (Section::CPU, "https://www.scoopgaming.com.tn/80-processeur"),
        (Section::GPU, "https://www.scoopgaming.com.tn/39-carte-graphique"),
        (Section::RAM, "https://www.scoopgaming.com.tn/131-memoire-pc"),
        (Section::MotherBoard, "https://www.scoopgaming.com.tn/40-carte-mere"),
        (Section::Storage, "https://www.scoopgaming.com.tn/15-stockage"),
        (Section::Cooler, "https://www.scoopgaming.com.tn/49-refroidissement"),
        (Section::Cooler, "https://www.scoopgaming.com.tn/42-ventilateur"),
        (Section::PSU, "https://www.scoopgaming.com.tn/48-alimentation"),
        (Section::Case, "https://www.scoopgaming.com.tn/38-boitier"),
    ]
};

pub struct ScoopGaming;

impl Site for ScoopGaming {
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