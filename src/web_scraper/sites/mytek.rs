use crate::utils::web_client::WebClientType;
use crate::web_scraper::product::{Product, ProductStatus};
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::{parse_price, ElementRefExt};
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};
use std::error::Error;
use std::str::FromStr;

static CONFIG: SiteConfig = SiteConfig {
    name: "Mytek",
    web_client_type: WebClientType::HttpClient,
    nav_sel: Lazy::new(|| Selector::parse("nav.custom-pagination ul.pagination li.page-item").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div#product-list-container div#seo-product-data div[data-id]").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("").unwrap())),
    desc_sel: Some(Lazy::new(|| Selector::parse("").unwrap())),
    page_desc_sel: None,
    sections: &[
        (Section::PC, "https://www.mytek.tn/informatique/ordinateur-de-bureau/pc-de-bureau.html"),
        (Section::GamingPC, "https://www.mytek.tn/informatique/ordinateur-de-bureau/ordinateur-gamer.html"),
        (Section::GamingPC, "https://www.mytek.tn/gaming/composant-pc-gamer/pack-gaming.html"),
        (Section::AllInOnePC, "https://www.mytek.tn/informatique/ordinateur-de-bureau/imac.html"),
        (Section::AllInOnePC, "https://www.mytek.tn/informatique/ordinateur-de-bureau/pc-tout-en-un.html"),
        (Section::Laptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-portable.html"),
        (Section::Laptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-portable-pro.html"),
        (Section::Laptop, "https://www.mytek.tn/informatique/ordinateurs-portables/ultrabook.html"),
        (Section::GamingLaptop, "https://www.mytek.tn/informatique/ordinateurs-portables/pc-gamer.html"),
        (Section::MacBook, "https://www.mytek.tn/informatique/ordinateurs-portables/mac.html"),
        (Section::Monitor, "https://www.mytek.tn/informatique/ordinateur-de-bureau/ecran.html"),
        (Section::Monitor, "https://www.mytek.tn/gaming/peripheriques-et-accessoires-gamers/ecran-gamer.html"),
        (Section::Mouse, "https://www.mytek.tn/gaming/peripheriques-et-accessoires-gamers/souris-gamer.html"),
        (Section::Keyboard, "https://www.mytek.tn/gaming/peripheriques-et-accessoires-gamers/clavier-gamer.html"),
        (Section::CPU, "https://www.mytek.tn/informatique/composants-informatique/processeur.html"),
        (Section::GPU, "https://www.mytek.tn/informatique/composants-informatique/carte-graphique.html"),
        (Section::Memory, "https://www.mytek.tn/gaming/composant-pc-gamer/barrette-memoire-gamer.html"),
        (Section::Memory, "https://www.mytek.tn/informatique/composants-informatique/barrettes-memoire.html"),
        (Section::Motherboard, "https://www.mytek.tn/informatique/composants-informatique/carte-mere.html"),
        (Section::Storage, "https://www.mytek.tn/informatique/stockage/disque-dur.html"),
        (Section::Cooler, "https://www.mytek.tn/gaming/composant-pc-gamer/refroidisseur-processeur-gamer.html"),
        (Section::Case, "https://www.mytek.tn/informatique/composants-informatique/boitier.html"),
        (Section::Case, "https://www.mytek.tn/gaming/composant-pc-gamer/boitier-pc-gamer.html"),
        (Section::PowerSupply, "https://www.mytek.tn/gaming/composant-pc-gamer/alimentation-pc-gamer.html"),
        (Section::PowerSupply, "https://www.mytek.tn/informatique/composants-informatique/bloc-d-alimentation.html"),
    ]
};

pub struct Mytek;

impl Site for Mytek {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_product(&self, section: Section, element: ElementRef) -> Result<Product, Box<dyn Error>> {
        let url = element.select_attr("data-url", "url")?;
        let title = element.select_attr("data-name", "title")?;
        let status = element.select_attr("data-erpstock", "status")?;
        let price = parse_price(&element.select_attr("data-price", "price")?)?;
        let final_price = parse_price(&element.select_attr("data-final-price", "final-price")?)?;

        let old_price = match price == final_price {
            false => Some(price),
            true => None,
        };

        let image = element.select_attr("data-image", "image")
            .map(|s| format!("https://www.mytek.tn/media/catalog/product{s}"))?;

        let description = match section.config().requires_description {
            true => Some(element.select_attr("data-description", "description")?),
            false => None
        };

        Ok(Product::new(
            self.name(), url, title, section, description, image,
            ProductStatus::from_str(&status)?, price, old_price
        )?)
    }
}