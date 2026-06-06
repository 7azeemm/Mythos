use std::sync::Arc;
use serde_json::Value;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::parsers::{DatasetEntry, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;

pub struct StorageParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Vec<DatasetEntry>
}

impl SectionParser for StorageParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Vec<DatasetEntry> {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b((?:\d+)\s*(?:g|go|gb|t|to|tb)|(?:256|500|512|1024))\b", &product.name) {
            if let Some(m) = caps.get(1) {
                product.specs["size"] = Value::String(m.as_str().to_string());
            }
        }
    }

    fn post_processing(&self, product: &mut Product) {
        let title = product.title.to_uppercase().replace(",", ".");

        if let Some(size) = product.specs.get("size").and_then(|s| s.as_str()) {
            let uppercase = size.to_uppercase();
            let number: u64 = uppercase.chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse().unwrap_or(0);

            let size = match uppercase.contains("T") {
                true => format!("{number}TB"),
                false if number == 1024 => "1TB".to_string(),
                false if number == 2048 => "2TB".to_string(),
                false => format!("{number}GB")
            };

            product.specs["size"] = Value::String(size.clone());
        }

        if (title.contains(" SSD") || title.contains("SPATIUM")) && !title.contains(" NVME") {
            product.specs["type"] = Value::String("SSD".to_string());
        } else if title.contains(" NVME") {
            product.specs["type"] = Value::String("NVMe".to_string());
        } else if title.contains(" HDD") || title.contains("TB") {
            product.specs["type"] = Value::String("HDD".to_string());
        }

        if title.contains("SATA") {
            product.specs["interface"] = Value::String("SATA".to_string());
        } else if title.contains("M.2") {
            product.specs["interface"] = Value::String("M.2".to_string());
        }

        let mut name = String::new();

        if let Some(size) = product.specs.get("size").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(&size);
        }
        if let Some(memory_type) = product.specs.get("type").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(memory_type);
        }
        if let Some(interface) = product.specs.get("interface").and_then(|s| s.as_str()) {
            name.push(' ');
            name.push_str(interface);
        }

        product.name.push_str(&name);
    }
}