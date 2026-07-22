use std::sync::Arc;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::monitor_parser::parse_display_specs;
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::SectionConfig;

pub struct TelevisionParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for TelevisionParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, text: &str) {
        parse_display_specs(product, text);
    }
}