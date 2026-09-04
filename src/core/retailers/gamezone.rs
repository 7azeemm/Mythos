use crate::core::product::ProductStatus;
use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::core::tracking::scrape_error::ProductParseError;
use crate::utils::scraper_ext::ElementRefExt;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};

static CONFIG: RetailerConfig = RetailerConfig {
    name: "gamezone",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.pagination li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products article").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.thumbnail-container img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.price").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span.regular-price").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-quantity").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.product-description p").unwrap())),
    empty_page_sel: None,
    sections: &[
        (Section::ConsoleGame, "https://gamezone.tn/jeux-ps5/"),
        (Section::ConsoleGame, "https://gamezone.tn/jeux-ps4/"),
        (Section::ConsoleGame, "https://gamezone.tn/jeux-nintendo-switch/"),
        (Section::ConsoleGame, "https://gamezone.tn/jeux-pc/"),
        (Section::ConsoleGame, "https://gamezone.tn/jeux-xbox/"),
    ],
};

pub struct GameZone;

impl Retailer for GameZone {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, ProductParseError> {
        let sel = self.config().status_sel.as_ref().unwrap();
        let status = element.select_text(sel, "status")?;
        if status.contains("Ajouter") {
            Ok(ProductStatus::InStock)
        } else if status.contains("Rupture") {
            Ok(ProductStatus::OutOfStock)
        } else {
            Err(ProductParseError::UnknownStatus { value: status })
        }
    }
}
