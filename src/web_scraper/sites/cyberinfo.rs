use std::error::Error;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[src]").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div[itemprop=description]").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span#product-availability").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "CyberInfo",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div.item-product").unwrap()),
    sections: &[
        (&Section::Laptop, "https://www.cyberinfo.tn/42-pc-portable"),
        (&Section::GamingLaptop, "https://www.cyberinfo.tn/190-pc-portable-gamer"),
        (&Section::PC, "https://www.cyberinfo.tn/43-pc-de-bureau"),
        (&Section::GamingPc, "https://www.cyberinfo.tn/191-pc-de-bureau-gamer-tunisie"),
        (&Section::GamingSetup, "https://www.cyberinfo.tn/211-full-setup-gamer-tunisie"),
        (&Section::PcAllInOne, "https://www.cyberinfo.tn/44-pc-all-in-one"),
        (&Section::Monitor, "https://www.cyberinfo.tn/45-ecran-pc"),
        (&Section::CPU, "https://www.cyberinfo.tn/58-processeur-tunisie"),
        (&Section::GPU, "https://www.cyberinfo.tn/60-carte-graphique-tunisie"),
        (&Section::RAM, "https://www.cyberinfo.tn/55-barette-memoire-ram-tunisie"),
        (&Section::MotherBoard, "https://www.cyberinfo.tn/56-carte-mere-tunisie"),
        (&Section::SSD, "https://www.cyberinfo.tn/67-disque-dur-ssd-tunisie"),
        (&Section::HDD, "https://www.cyberinfo.tn/65-disque-dur-interne-tunisie"),
        (&Section::Cooler, "https://www.cyberinfo.tn/50-refroidisseur-pc-tunisie"),
        (&Section::Cooler, "https://www.cyberinfo.tn/59-ventilateur-refroidisseur-pc-tunisie"),
        (&Section::Case, "https://www.cyberinfo.tn/192-boitier-pc"),
        (&Section::PSU, "https://www.cyberinfo.tn/193-boite-alimentation-tunisie"),
    ]
};

pub struct CyberInfo;

impl Site for CyberInfo {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: &Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let title = element.select(&TITLE_SEL).next().ok_or("title not found")?;
        let image = element.select(&IMAGE_SEL).next().ok_or("image not found")?;
        let status = element.select(&STATUS_SEL).next().ok_or("status not found")?.get_text();
        let price = parse_price(&element.select(&PRICE_SEL).next().ok_or("price not found")?.get_text())?;

        let description = match section.requires_description() {
            false => vec![],
            true => element.select(&DESCRIPTION_SEL)
                .next()
                .ok_or("description not found")?
                .get_text()
                .split("-")
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        };

        let url = title
            .value()
            .attr("href")
            .ok_or("url not found")?
            .to_string();

        let title = title.get_text();

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
            title,
            source: self.name().to_string(),
            sections: vec![section.to_str().to_string()],
            description,
            image,
            in_stock: status == "En Stock",
            regular_price,
            price,
            history: Default::default(),
            added_at: None,
            updated_at: None,
            created_at: Default::default(),
        })
    }
}