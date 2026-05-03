use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use scraper::{ElementRef, Selector};
use std::error::Error;
use once_cell::sync::Lazy;
use crate::web_scraper::scheduler::print;
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.font-heading").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("img[alt]").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.flex-wrap span").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("p.font-heading").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("p.line-through").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "SkyMil-Shop",
    web_client_type: WebClientType::Browser,
    nav_selector: Lazy::new(|| Selector::parse("nav[role=navigation] ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.card-product").unwrap()),
    sections: &[
        (&Section::CPU, "https://www.skymil-shop.com/catalogue/composants/processeur-intel"),
        (&Section::CPU, "https://www.skymil-shop.com/catalogue/composants/processeur-amd"),
    ],
};

pub struct SkyMilShop;

impl Site for SkyMilShop {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?;
        let price = element.select(&PRICE_SEL).next().ok_or("price not found")?;

        let description = match section.requires_description() {
            false => vec![],
            true => {
                todo!()
            }
        };

        let url = title
            .value()
            .attr("href")
            .ok_or("url not found")
            .map(|link| format!("https://www.skymil-shop.com/{link}"))?;

        let image = image
            .value()
            .attr("src")
            .ok_or("image url not found")?
            .to_string();

        let regular_price = match element.select(&REGULAR_PRICE_SEL).next() {
            Some(p) => Some(parse_price(&p.get_text())?),
            None => None,
        };

        Ok(Product {
            id: parse_url(self.name(), &url),
            url,
            title: title.get_text(),
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status.get_text() == "In stock",
            regular_price,
            price: parse_price(&price.get_text())?,
            history: Default::default(),
            added_at: None,
            updated_at: None,
            created_at: Default::default(),
        })
    }
}