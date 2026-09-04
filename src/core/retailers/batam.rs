use crate::core::retailers::{Retailer, RetailerConfig};
use crate::core::sections::Section;
use crate::core::tracking::scrape_error::PaginationError;
use crate::utils::web_client::WebClientType;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};

static CONFIG: RetailerConfig = RetailerConfig {
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
    empty_page_sel: None,
    sections: &[
        (Section::PC, "https://batam.com.tn/informatique/ordinateur-de-bureau/ordinateur-de-bureau.html"),
        (Section::Laptop, "https://batam.com.tn/informatique/ordinateur-portable.html"),
        (Section::GamingLaptop, "https://batam.com.tn/gaming/pc-gaming/pc-portable-gamer.html"),
        (Section::Monitor, "https://batam.com.tn/informatique/ordinateur-de-bureau/ecran-pc.html"),
        (Section::Keyboard, "https://batam.com.tn/gaming/peripheriques-et-accessoires-gaming/clavier-gamer.html"),
        (Section::Mouse, "https://batam.com.tn/gaming/peripheriques-et-accessoires-gaming/souris-et-tapis-gamer.html"),
        (Section::Headphones, "https://batam.com.tn/gaming/peripheriques-et-accessoires-gaming/casque-et-ecouteur-gamer.html"),
        (Section::Console, "https://batam.com.tn/gaming/console-de-jeux/console-de-jeux.html"),
        (Section::Controller, "https://batam.com.tn/gaming/peripheriques-et-accessoires-gaming/manette.html"),
        (Section::ConsoleGame, "https://batam.com.tn/gaming/console-de-jeux/jeux-video.html"),
        (Section::ConsoleAccessories, "https://batam.com.tn/gaming/peripheriques-et-accessoires-gaming/volants.html"),
        (Section::Smartphone, "https://batam.com.tn/telephonie-et-montres-connectes/smartphone-et-telephone-portable/smartphone.html"),
        (Section::Smartphone, "https://batam.com.tn/telephonie-et-montres-connectes/smartphone-et-telephone-portable/iphone.html"),
        (Section::Tablet, "https://batam.com.tn/informatique/tablette.html"),
        (Section::Smartwatch, "https://batam.com.tn/telephonie-et-montres-connectes/montres-connectes.html"),
        (Section::Television, "https://batam.com.tn/tv-image-son/televiseurs.html"),
    ],
};

pub struct Batam;

impl Retailer for Batam {
    fn config(&self) -> &RetailerConfig {
        &CONFIG
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, PaginationError> {
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
