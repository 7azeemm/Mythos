use std::sync::Arc;
use serde_json::Value;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::parsers::{DatasetEntry, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;

pub struct MemoryParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Vec<DatasetEntry>
}

impl SectionParser for MemoryParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Vec<DatasetEntry> {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d+)\s*(?:gb|go|g)\b", &product.name) {
            if let Some(m) = caps.get(1) {
                product.specs["size"] = Value::String(m.as_str().to_string());
            }
        }

        if let Some(caps) = RegexCache::captures(r"(?i)\b(ddr[345])\b", &product.name) {
            if let Some(m) = caps.get(1) {
                product.specs["memory_type"] = Value::String(m.as_str().to_string());
            }
        }

        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d{4})", &product.name) {
            if let Some(m) = caps.get(1) {
                product.specs["speed"] = Value::String(m.as_str().to_string());
            }
        }
    }

    fn post_processing(&self, product: &mut Product) {
        if let Some(size) = product.specs.get("size").and_then(|s| s.as_str()) {
            product.specs["size"] = Value::String(format!("{size}GB"));
        }

        if let Some(speed) = product.specs.get("speed").and_then(|s| s.as_str()) {
            if let Ok(speed) = speed.parse::<u32>() {
                if product.specs.get("memory_type").is_none() {
                    product.specs["memory_type"] = Value::String(match speed {
                        0..1866 => "DDR3",
                        1866..4800 => "DDR4",
                        4800..10000 => "DDR5",
                        10000..=u32::MAX => "",
                    }.to_string());
                }
                product.specs["speed"] = Value::String(format!("{speed}Mhz"));
            }
        }

        let mut name = String::new();
        
        if let Some(size) = product.specs.get("size").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(&size);
        }

        if let Some(memory_type) = product.specs.get("memory_type").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(&memory_type);
        }

        if let Some(speed) = product.specs.get("speed").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(&speed);
        }

        product.name.push_str(&name);
    }
}