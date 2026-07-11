use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::utils::{extract_basics, validate_url};
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Html, Selector};
use crate::utils::scraper_ext::ElementRefExt;

static CONFIG: SiteConfig = SiteConfig {
    name: "SkyMil-Shop",
    web_client_type: WebClientType::Browser,
    nav_sel: Lazy::new(|| Selector::parse("nav[role=navigation] ul li").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("div.card-product").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("a.font-heading").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("img[alt]").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("p.font-heading").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("p.line-through").unwrap()),
    price_sel_2: None,
    status_sel: Some(Lazy::new(|| Selector::parse("div.flex-wrap span").unwrap())),
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("p.text-\\[13px\\].text-muted-foreground").unwrap())),
    empty_page_sel: Some(Lazy::new(|| Selector::parse("main#main-content div.container div.flex div.flex-1 div.text-center").unwrap())),
    sections: &[
        (Section::PC, "https://www.skymil-shop.com/catalogue/equipement-pro/workstation-intel"),
        (Section::PC, "https://www.skymil-shop.com/catalogue/equipement-pro/workstation-amd"),
        (Section::PC, "https://www.skymil-shop.com/catalogue/equipement-pro/bureautique"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/pc-gamer-intel"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/pc-gamer-amd"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/watercooled-pc"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/powered-by-msi"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/powered-by-asus"),
        (Section::GamingPC, "https://www.skymil-shop.com/catalogue/pc-gamer-bureautique/full-setup"),
        (Section::AllInOnePC, "https://www.skymil-shop.com/catalogue/equipement-pro/all-in-one"),
        (Section::Laptop, "https://www.skymil-shop.com/catalogue/pc-portable/pc-portable-pro"),
        (Section::GamingLaptop, "https://www.skymil-shop.com/catalogue/pc-portable/pc-portable-gamer"),
        (Section::Monitor, "https://www.skymil-shop.com/catalogue/ecran/ecrans-pro"),
        (Section::Monitor, "https://www.skymil-shop.com/catalogue/ecran/ecran-gamer"),
        (Section::CPU, "https://www.skymil-shop.com/catalogue/composants/processeur-intel"),
        (Section::CPU, "https://www.skymil-shop.com/catalogue/composants/processeur-amd"),
        (Section::GPU, "https://www.skymil-shop.com/catalogue/composants/carte-graphique"),
        (Section::Memory, "https://www.skymil-shop.com/catalogue/composants/barrette-memoire"),
        (Section::Memory, "https://www.skymil-shop.com/catalogue/pc-portable/ram-pour-pc-portable"),
        (Section::Storage, "https://www.skymil-shop.com/catalogue/composants/disque-dur-ssd-nvme"),
        (Section::Motherboard, "https://www.skymil-shop.com/catalogue/composants/carte-mere-intel"),
        (Section::Motherboard, "https://www.skymil-shop.com/catalogue/composants/carte-mere-amd"),
        (Section::Cooler, "https://www.skymil-shop.com/catalogue/composants/aircooling"),
        (Section::Cooler, "https://www.skymil-shop.com/catalogue/composants/watercooling"),
        (Section::PowerSupply, "https://www.skymil-shop.com/catalogue/composants/bloc-alimentation"),
        (Section::Case, "https://www.skymil-shop.com/catalogue/composants/boitier"),
        (Section::Mouse, "https://www.skymil-shop.com/catalogue/equipement-pro/souris-pro"),
        (Section::Keyboard, "https://www.skymil-shop.com/catalogue/equipement-pro/clavier-pro"),
        (Section::MousePad, "https://www.skymil-shop.com/catalogue/peripheriques-gaming/tapis-gamer"),
        (Section::Headphones, "https://www.skymil-shop.com/catalogue/peripheriques-gaming/micro-casque-gamer"),
        (Section::GamingChair, "https://www.skymil-shop.com/catalogue/peripheriques-gaming/siege-gamer"),
        (Section::AccessoriesCombo, "https://www.skymil-shop.com/catalogue/equipement-pro/ensemble-pro"),
        (Section::UpgradeKit, "https://www.skymil-shop.com/catalogue/pack-level-up"),
        (Section::Controller, "https://www.skymil-shop.com/catalogue/peripheriques-gaming/manette-de-jeux"),
        (Section::ConsoleAccessories, "https://www.skymil-shop.com/catalogue/peripheriques-gaming/racing-wheel"),
    ],
};

pub struct SkyMilShop;

impl Site for SkyMilShop {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_basics(&self, element: ElementRef) -> Result<(String, String, String), String> {
        let config = self.config();
        let (title, url, image) = extract_basics(element, &config.title_sel, &config.image_sel)?;
        let url = format!("https://www.skymil-shop.com{url}");
        
        validate_url(&url)?;
        validate_url(&image)?;
        
        Ok((title, url, image))
    }

    fn check_if_page_empty(&self, doc: &Html) -> bool {
        if let Some(sel) = &self.config().empty_page_sel {
            if let Some(elem) = doc.select(sel).next() {
                let text = elem.get_text();
                if text.contains("No product found") || text.contains("Aucun produit trouvé") {
                    return true
                }
            }
        }
        false
    }
}