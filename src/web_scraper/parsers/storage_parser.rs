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

    fn parse_specs(&self, product: &mut Product, cleaned_title: &str) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b((?:\d+)\s*(?:g|go|gb|t|to|tb)|(?:256|500|512|1024))\b", cleaned_title) {
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
        if (title.contains(" SSD") || title.contains("SPATIUM")) && !title.contains(" NVME") {
            product.filter_ids.insert("storage_type".to_string(), "SSD".to_string());
        } else if title.contains(" NVME") {
            product.filter_ids.insert("storage_type".to_string(), "NVMe".to_string());
        } else if title.contains(" HDD") || title.contains("TB") {
            product.filter_ids.insert("storage_type".to_string(), "HDD".to_string());
        }

        if title.contains("SATA") {
            product.filter_ids.insert("storage_interface".to_string(), "SATA".to_string());
        } else if title.contains("M.2") {
            product.filter_ids.insert("storage_interface".to_string(), "M.2".to_string());
        }
    }

    fn post_processing(&self, product: &mut Product) -> Option<String> {
        let mut specs = Vec::new();

        if let Some(size) = product.filter_ids.get("size") {
            specs.push(size.clone());
        }
        if let Some(memory_type) = product.filter_ids.get("type") {
            specs.push(memory_type.clone());
        }
        if let Some(interface) = product.filter_ids.get("interface") {
            specs.push(interface.clone());
        }

        Some(specs.join(" "))
    }
}