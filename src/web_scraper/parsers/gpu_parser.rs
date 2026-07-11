use crate::utils::regex_cache::RegexCache;
use crate::utils::serde_ext::JsonExt;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::{SectionConfig, SectionParser};
use crate::web_scraper::product::Product;
use std::sync::Arc;

pub struct GPUParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset,
}

impl SectionParser for GPUParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, cleaned_title: &str) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d+)\s*(?:gb|go|g)\b", cleaned_title) {
            if let Some(m) = caps.get(1) {
                product.filter_ids.insert("memory_size".to_string(), m.as_str().to_string());
            }
        }
    }

    fn post_processing(&self, product: &mut Product) -> Option<String> {
        if let Some(memory_size) = product.filter_ids.get_mut("memory_size") {
            memory_size.push_str("GB");
        }
        None
    }
}