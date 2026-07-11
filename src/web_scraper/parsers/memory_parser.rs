use std::sync::Arc;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::{SectionConfig, SectionParser};
use crate::web_scraper::product::Product;

pub struct MemoryParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for MemoryParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, cleaned_title: &str) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d+)\s*(?:gb|go|g)\b", cleaned_title) {
            if let Some(size) = caps.get(1) {
                product.filter_ids.insert("memory_size".to_string(), format!("{}GB", size.as_str()));
            }
        }

        if let Some(caps) = RegexCache::captures(r"(?i)\b(ddr[345])\b", cleaned_title) {
            if let Some(m) = caps.get(1) {
                product.filter_ids.insert("memory_type".to_string(), m.as_str().to_uppercase());
            }
        }

        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d{4})", cleaned_title) {
            if let Some(m) = caps.get(1) {
                if let Ok(speed) = m.as_str().parse::<u32>() {
                    if !product.filter_ids.contains_key("memory_type") && speed <= 10000 {
                        product.filter_ids.insert("memory_type".to_string(), match speed {
                            0..1866 => "DDR3",
                            1866..4800 => "DDR4",
                            4800..10000 => "DDR5",
                            10000..=u32::MAX => "",
                        }.to_string());
                    }
                    product.filter_ids.insert("memory_speed".to_string(), format!("{speed}Mhz"));
                }
            }
        }
    }

    fn post_processing(&self, product: &mut Product) -> Option<String> {
        let mut specs = Vec::new();

        if let Some(size) = product.filter_ids.get("size") {
            specs.push(size.clone());
        }

        if let Some(memory_type) = product.filter_ids.get("memory_type") {
            specs.push(memory_type.clone());
        }

        if let Some(speed) = product.filter_ids.get("speed") {
            specs.push(speed.clone());
        }

        Some(specs.join(" "))
    }
}