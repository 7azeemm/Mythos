use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use crate::utils::scraper_ext::ElementRefExt;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::ProductStatus;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};

static CONFIG: SiteConfig = SiteConfig {
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
        (Section::Console, "https://gameszone.tn/11-consoles-ps5"),
        (Section::Console, "https://gameszone.tn/13-consoles-ps4"),
        (Section::Console, "https://gameszone.tn/55-nintendo-switch-1"),
        (Section::Console, "https://gameszone.tn/56-nintendo-switch-2"),
        (Section::Console, "https://gameszone.tn/22-consoles-xbox"),
        (Section::Console, "https://gameszone.tn/36-consoles-portable"),
        (Section::Controller, "https://gameszone.tn/10-accessoires-ps5"),
        (Section::Controller, "https://gameszone.tn/16-accessoires-ps4"),
        (Section::Controller, "https://gameszone.tn/53-accessoires-xbox"),
        (Section::ConsoleGame, "https://gameszone.tn/47-top-jeux"),
        (Section::ConsoleGame, "https://gameszone.tn/4-jeux-ps5"),
        (Section::ConsoleGame, "https://gameszone.tn/15-jeux-ps4"),
        (Section::ConsoleGame, "https://gameszone.tn/49-jeux-xbox"),
        (Section::ConsoleAccessories, "https://gameszone.tn/10-accessoires-ps5"),
        (Section::ConsoleAccessories, "https://gameszone.tn/16-accessoires-ps4"),
        (Section::ConsoleAccessories, "https://gameszone.tn/53-accessoires-xbox"),
    ],
};

pub struct GameZone;

impl Site for GameZone {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_status(&self, element: ElementRef) -> Result<ProductStatus, String> {
        let sel = self.config().status_sel.as_ref().unwrap();
        let status = element.select_text(sel, "status")?;
        if status.contains("Ajouter") {
            Ok(ProductStatus::InStock)
        } else if status.contains("Rupture") {
            Ok(ProductStatus::OutOfStock)
        } else {
            Err("status not found".to_string())
        }
    }
}