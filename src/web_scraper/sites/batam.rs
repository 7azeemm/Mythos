use std::error::Error;
use chrono::Utc;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-item-link").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-item-photo img[src]").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-info span").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span[data-price-type=finalPrice]").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span[data-price-type=oldPrice]").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "Batam",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav[aria-label=pagination] ol li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products ul li form.product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://batam.com.tn/informatique/ordinateur-portable.html"),
        (&Section::GamingLaptop, "https://batam.com.tn/gaming/pc-gaming/pc-portable-gamer.html"),
        (&Section::PC, "https://batam.com.tn/informatique/ordinateur-de-bureau/ordinateur-de-bureau.html"),
        (&Section::Monitor, "https://batam.com.tn/informatique/ordinateur-de-bureau/ecran-pc.html"),
    ]
};

pub struct Batam;

impl Site for Batam {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;
        let url = title.attr("href").ok_or("url not found")?.to_string();
        let title = title.get_text();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        let description = match section.requires_description() {
            true => vec![],
            false => todo!(),
        };

        let image = image
            .value()
            .attr("src")
            .ok_or("image url not found")?
            .to_string();

        println!("{url}, {title}, {status}, {image}, {price}, {regular_price:?}, {description:?}");

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title,
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status == "En stock",
            price,
            regular_price,
            history: Value::Array(vec![]),
            added_at: None,
            updated_at: None,
            created_at: Utc::now(),
        })
    }

    fn parse_page_count(&self, doc: &Html) -> Result<i32, Box<dyn Error>> {
        let elements = doc.select(&self.config().nav_selector).collect::<Vec<ElementRef>>();
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