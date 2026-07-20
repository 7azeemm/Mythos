use std::sync::Arc;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::SectionConfig;

pub struct ConsoleGameParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for ConsoleGameParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, text: &str) {
        let upper = text.to_uppercase();

        let platform = if upper.contains("PLAYSTATION 5") || upper.contains("PS5") {
            "PlayStation 5"
        } else if upper.contains("PLAYSTATION 4") || upper.contains("PS4") {
            "PlayStation 4"
        } else if upper.contains("PLAYSTATION 3") || upper.contains("PS3") {
            "PlayStation 3"
        } else if upper.contains("XBOX") {
            "Xbox"
        } else if RegexCache::custom_match("PC", &upper) {
            "PC"
        } else if upper.contains("NINTENDO") || upper.contains("SWITCH") {
            "Nintendo Switch"
        } else {
            "Others"
        };

        product.filter_ids.insert("platform".to_string(), platform.to_string());
    }
}