use crate::utils::web_client::WebClientType;
use crate::web_scraper::sections::Section;
use crate::web_scraper::sites::utils::{validate_url, ElementRefExt};
use crate::web_scraper::sites::{Site, SiteConfig};
use once_cell::sync::Lazy;
use scraper::{ElementRef, Selector};

static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a[href]").unwrap());

static CONFIG: SiteConfig = SiteConfig {
    name: "MegaPC",
    web_client_type: WebClientType::Browser,
    nav_sel: Lazy::new(|| Selector::parse("button.rounded-md.bg-gray-200").unwrap()),
    product_sel: Lazy::new(|| Selector::parse("article.product-card").unwrap()),
    title_sel: Lazy::new(|| Selector::parse("p.text-skin-base").unwrap()),
    image_sel: Lazy::new(|| Selector::parse("div.card-img-container img").unwrap()),
    price_sel: Lazy::new(|| Selector::parse("span.inline-block").unwrap()),
    old_price_sel: Lazy::new(|| Selector::parse("del.text-sm").unwrap()),
    price_sel_2: None,
    status_sel: None,
    desc_sel: None,
    page_desc_sel: Some(Lazy::new(|| Selector::parse("div.productView-info").unwrap())),
    sections: &[
        (Section::GamingPC, "https://megapc.tn/shop/ORDINATEURS/PC%20GAMER"),
        (Section::GamingPC, "https://megapc.tn/shop/ORDINATEURS/LEGENDARY"),
        (Section::GamingPC, "https://megapc.tn/shop/search/PREBUILD"),
        (Section::GamingSetup, "https://megapc.tn/shop/ORDINATEURS/FULL%20SETUP"),
        (Section::AllInOnePC, "https://megapc.tn/shop/ORDINATEURS/PC%20TOUT%20EN%20UN"),
        (Section::PC, "https://megapc.tn/shop/ORDINATEURS/BAREBONE"),
        (Section::PC, "https://megapc.tn/shop/ORDINATEURS/PRO%20PC"),
        (Section::GamingLaptop, "https://megapc.tn/shop/PC%20PORTABLE/PC%20PORTABLE%20GAMER"),
        (Section::ProLaptop, "https://megapc.tn/shop/PC%20PORTABLE/PC%20PORTABLE%20PRO"),
        (Section::Monitor, "https://megapc.tn/shop/ECRANS/ECRANS%20GAMING"),
        (Section::Monitor, "https://megapc.tn/shop/ECRANS/ECRANS%20PRO"),
        (Section::CPU, "https://megapc.tn/shop/COMPOSANTS/PROCESSEUR"),
        (Section::GPU, "https://megapc.tn/shop/COMPOSANTS/CARTE%20GRAPHIQUE"),
        (Section::RAM, "https://megapc.tn/shop/COMPOSANTS/BARETTE%20M%C3%89MOIRE"),
        (Section::MotherBoard, "https://megapc.tn/shop/COMPOSANTS/CARTE%20M%C3%88RE"),
        (Section::Storage, "https://megapc.tn/shop/STOCKAGE/DISQUE-SSD"),
        (Section::Storage, "https://megapc.tn/shop/STOCKAGE/DISQUE-NVME"),
        (Section::Storage, "https://megapc.tn/shop/STOCKAGE/DISQUE-HDD"),
        (Section::Cooler, "https://megapc.tn/shop/COMPOSANTS/REFROIDISSEMENT"),
        (Section::PSU, "https://megapc.tn/shop/COMPOSANTS/ALIMENTATION"),
        (Section::Case, "https://megapc.tn/shop/COMPOSANTS/BOITIER"),
    ]
};

pub struct MegaPC;

impl Site for MegaPC {
    fn config(&self) -> &SiteConfig {
        &CONFIG
    }

    fn parse_basics(&self, element: ElementRef) -> Result<(String, String, String), String> {
        let config = self.config();
        let title = element.select_text(&config.title_sel, "title")?;

        let url = element.select_elem(&URL_SEL, "url")?.select_attr("href", "url")?;
        let url = format!("https://megapc.tn{url}");
        validate_url(&url)?;

        let image = element.select_elem(&config.image_sel, "image")?.select_attr("src", "image url")?;
        let image = format!("https://megapc.tn{image}");
        validate_url(&image)?;

        Ok((title, url, image))
    }
}