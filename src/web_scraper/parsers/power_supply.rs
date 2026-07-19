use std::sync::Arc;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::SectionConfig;

pub struct PowerSupplyParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for PowerSupplyParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, text: &str) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d{3,4})\s*(?:w|watt|watts)\b", text) {
            if let Some(m) = caps.get(1) {
                product.filter_ids.insert("wattage".to_string(), format!("{} Watts", m.as_str()));
            }
        }
    }
}