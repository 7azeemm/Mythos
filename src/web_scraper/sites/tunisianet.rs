use std::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::Selector;
use crate::web_scraper::sites::Section;

pub mod fetcher;

static SECTIONS: &[(Section, &str)] = &[
    (Section::PC, "https://www.tunisianet.com.tn/373-pc-de-bureau"),
    (Section::GamingPc, "https://www.tunisianet.com.tn/682-pc-de-bureau-gamer"),
    (Section::PcAllInOne, "https://www.tunisianet.com.tn/686-pc-tout-en-un"),
    (Section::GamingSetup, "https://www.tunisianet.com.tn/732-full-setup-gamer"),
    (Section::Laptop, "https://www.tunisianet.com.tn/301-pc-portable-tunisie"),
    (Section::GamingLaptop, "https://www.tunisianet.com.tn/681-pc-portable-gamer"),
    (Section::ProLaptop, "https://www.tunisianet.com.tn/703-pc-portable-pro"),
];

const MONITOR_URL: &str = "https://www.tunisianet.com.tn/667-ecran-pc-tunisie";
const MOUSE_URL: &str = "https://www.tunisianet.com.tn/334-souris-informatique";
const KEYBOARD_URL: &str = "https://www.tunisianet.com.tn/704-claviers";

const CPU_URL: &str = "https://www.tunisianet.com.tn/421-processeur";
const RAM_URL: &str = "https://www.tunisianet.com.tn/409-barrette-memoire";
const MOTHERBOARD_URL: &str = "https://www.tunisianet.com.tn/420-carte-mere";
const GPU_URL: &str = "https://www.tunisianet.com.tn/410-carte-graphique-tunisie";
const PSU_URL: &str = "https://www.tunisianet.com.tn/423-boite-alimentation-pc-tunisie";
const CASE_URL: &str = "https://www.tunisianet.com.tn/425-boitier";
const HDD_URL: &str = "https://www.tunisianet.com.tn/408-disque-dur-interne";
const SDD_URL: &str = "https://www.tunisianet.com.tn/379-disques-ssd";
const FAN_AND_COOLER_URL: &str = "https://www.tunisianet.com.tn/427-refroidisseur-ventilateur-boitier";

static PRODUCTS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.products").unwrap());
static PRODUCT_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.item-product").unwrap());
static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title").unwrap());
static REF_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.product-reference").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[data-full-size-image-url]").unwrap());
static DESC_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse(r#"div.product-description div[itemprop="description"]"#).unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div#stock_availability").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());

static ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/(\d+)-").unwrap());