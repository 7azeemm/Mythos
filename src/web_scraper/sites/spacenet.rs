use std::error::Error;
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::Product;
use crate::web_scraper::scheduler::print;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::{Site, SiteConfig};
use crate::web_scraper::sites::utils::{parse_price, parse_url, ElementRefExt};

static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product_name a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("img.product_image").unwrap());
static DESCRIPTION_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.decriptions-short").unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.product-quantities label").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());
static REGULAR_PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.regular-price").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "SpaceNet",
    web_client_type: WebClientType::HttpClient,
    nav_selector: Lazy::new(|| Selector::parse("nav.pagination ul.page-list li").unwrap()),
    product_selector: Lazy::new(|| Selector::parse("div.products div#box-product-list div.item-product-list").unwrap()),
    sections: &[
        (&Section::Laptop, "https://spacenet.tn/18-ordinateur-portable"),
        (&Section::GamingLaptop, "https://spacenet.tn/204-pc-portable-gamer-tunisie"),
        (&Section::ProLaptop, "https://spacenet.tn/321-pc-portables-pro-tunisie"),
        (&Section::PC, "https://spacenet.tn/73-ordinateur-bureau-tunisie"),
        (&Section::PcAllInOne, "https://spacenet.tn/80-pc-tout-en-un-tunisie"),
        (&Section::GamingPc, "https://spacenet.tn/205-ordinateur-de-bureau-gamer-tunisie"),
        (&Section::GamingSetup, "https://spacenet.tn/1390-setup-gaming"),
        (&Section::Monitor, "https://spacenet.tn/388-ecran-gamer-tunisie"),
        (&Section::Monitor, "https://spacenet.tn/1142-ecrans-professionnels"),
        (&Section::CPU, "https://spacenet.tn/399-processeur"),
        (&Section::GPU, "https://spacenet.tn/397-cartes-graphiques"),
        (&Section::RAM, "https://spacenet.tn/398-memoires-ram"),
        (&Section::MotherBoard, "https://spacenet.tn/394-cartes-meres"),
        (&Section::SSD, "https://spacenet.tn/395-disque-dur-ssd-hdd-tunisie"),
        (&Section::Case, "https://spacenet.tn/393-boitier"),
        (&Section::PSU, "https://spacenet.tn/724-bloc-d-alimentation"),
        (&Section::Cooler, "https://spacenet.tn/726-ventilateur"),
        (&Section::Cooler, "https://spacenet.tn/744-refroidisseur-pc-bureau"),
    ],
};

pub struct SpaceNet;

impl Site for SpaceNet {
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
            in_stock: status == "En stock",
            regular_price,
            price,
            history: Default::default(),
            added_at: None,
            updated_at: None,
            created_at: Default::default(),
        })
    }
}