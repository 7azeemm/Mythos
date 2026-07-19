use std::sync::Arc;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::{SectionConfig, SectionParser};
use crate::web_scraper::product::Product;

pub struct StorageParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for StorageParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, text: &str) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b((?:\d+)\s*(?:g|go|gb|t|to|tb)|(?:256|500|512|1024))\b", text) {
            if let Some(m) = caps.get(1) {
                let size = m.as_str().to_uppercase();
                let number: u64 = size.chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse().unwrap_or(0);

                let size = match size.contains("T") {
                    true => format!("{number}TB"),
                    false if number == 1024 => "1TB".to_string(),
                    false if number == 2048 => "2TB".to_string(),
                    false => format!("{number}GB")
                };

                product.filter_ids.insert("storage_size".to_string(), size);
            }
        }

        let title = product.title.to_uppercase().replace(",", ".");
        if title.contains(" NVME") || title.contains("M.2") || title.contains("PCIE") {
            product.filter_ids.insert("storage_type".to_string(), "NVME".to_string());
        } else if title.contains(" SSD") || title.contains("SPATIUM") {
            product.filter_ids.insert("storage_type".to_string(), "SSD".to_string());
        } else {
            product.filter_ids.insert("storage_type".to_string(), "HDD".to_string());
        }
    }

    fn post_processing(&self, product: &mut Product) -> Option<String> {
        let mut specs = Vec::new();

        if let Some(size) = product.filter_ids.get("storage_size") {
            specs.push(size.clone());
        }
        if let Some(memory_type) = product.filter_ids.get("storage_type") {
            specs.push(memory_type.clone());
        }

        Some(specs.join(" "))
    }
}