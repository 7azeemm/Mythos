use crate::core::dataset::Dataset;
use crate::core::parsers::SectionParser;
use crate::core::parsers::monitor_parser::parse_display_specs;
use crate::core::product::Product;
use crate::core::sections::SectionConfig;
use std::sync::Arc;

pub struct TelevisionParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset,
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