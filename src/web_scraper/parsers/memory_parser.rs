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

    fn parse_specs(&self, product: &mut Product, text: &str) {
        let laptop = vec!["SO-DIMM", "SO DIMM", "POUR NB", "SODIM", "PORTABLE"];
        let upper = text.to_uppercase();

        product.filter_ids.insert("platform".to_string(), if laptop.iter().any(|v| upper.contains(v)) {
            "Laptop".to_string()
        } else {
            "Desktop".to_string()
        });

        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d+)\s*(?:gb|go|g)\b", text) {
            if let Some(size) = caps.get(1) {
                product.filter_ids.insert("memory_size".to_string(), format!("{}GB", size.as_str()));
            }
        }

        if let Some(caps) = RegexCache::captures(r"(?i)\b(ddr[345])\b", text) {
            if let Some(m) = caps.get(1) {
                product.filter_ids.insert("memory_type".to_string(), m.as_str().to_uppercase());
            }
        }

        let mut memory_speed = None;

        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d{3,5})\s*(?:MHz|MT|Hz)", text) {
            if let Some(m) = caps.get(1) {
                if let Ok(speed) = m.as_str().parse::<u32>() && speed <= 10000 {
                    memory_speed = Some(speed);
                    product.filter_ids.insert("memory_speed".to_string(), format!("{speed} MT/s"));
                }
            }
        }

        if !product.filter_ids.contains_key("memory_speed") {
            for speed in vec![2400, 2666, 3200, 3600, 5200, 5600] {
                if text.contains(&speed.to_string()) {
                    memory_speed = Some(speed);
                    product.filter_ids.insert("memory_speed".to_string(), format!("{speed} MT/s"));
                    break
                }
            }
        }

        if !product.filter_ids.contains_key("memory_type") && let Some(speed) = memory_speed {
            product.filter_ids.insert("memory_type".to_string(), match speed {
                0..1866 => "DDR3",
                1866..4800 => "DDR4",
                4800..10000 => "DDR5",
                10000..=u32::MAX => "",
            }.to_string());
        }
    }

    fn post_processing(&self, product: &mut Product) -> Option<String> {
        let mut specs = Vec::new();

        if let Some(size) = product.filter_ids.get("memory_size") {
            specs.push(size.clone());
        }

        if let Some(memory_type) = product.filter_ids.get("memory_type") {
            specs.push(memory_type.clone());
        }

        if let Some(speed) = product.filter_ids.get("memory_speed") {
            specs.push(speed.clone());
        }

        Some(specs.join(" "))
    }
}