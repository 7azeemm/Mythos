use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

static CONFIG: SiteConfig = SiteConfig {
    name: "Batam",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav[aria-label=pagination] ol li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.products ul li form.product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("a.product-item-link").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("a.product-item-photo img[src]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span[data-price-type=finalPrice]").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("span[data-price-type=oldPrice]").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.product-info span").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.product-description").unwrap())),
    sections: &[
        (Section::Laptop, "https://batam.com.tn/informatique/ordinateur-portable.html"),
        (Section::GamingLaptop, "https://batam.com.tn/gaming/pc-gaming/pc-portable-gamer.html"),
        (Section::PC, "https://batam.com.tn/informatique/ordinateur-de-bureau/ordinateur-de-bureau.html"),
        (Section::Monitor, "https://batam.com.tn/informatique/ordinateur-de-bureau/ecran-pc.html"),
    ]
};

pub struct Batam;

impl Site for Batam {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let elements = doc.select(&self.config().nav_sel).collect::<Vec<ElementRef>>();
        let len = elements.len();
        if len == 0 || len == 1 || len == 2 {
            return Ok(1);
        }
        Ok((len - 2) as i32)
    }

    fn format_url(&self, url: &str, page: i32) -> String {
        format!("{url}?p={page}")
    }
}