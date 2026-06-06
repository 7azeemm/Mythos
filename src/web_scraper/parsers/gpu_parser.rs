use std::sync::Arc;
use serde_json::Value;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::parsers::{DatasetEntry, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::ChipsetEntry;

pub struct GPUParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Vec<DatasetEntry>,
    pub chipsets: Vec<ChipsetEntry>
}

impl SectionParser for GPUParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Vec<DatasetEntry> {
        &self.dataset
    }

    fn chipsets(&self) -> &[ChipsetEntry] {
        self.chipsets.as_slice()
    }

    fn parse_specs(&self, product: &mut Product) {
        if let Some(caps) = RegexCache::captures(r"(?i)\b(\d+)\s*(?:gb|go|g)\b", &product.name) {
            if let Some(m) = caps.get(1) {
                product.specs["memory_size"] = Value::String(m.as_str().to_string());
            }
        }
    }

    // fn get_simple_name(&self, product: &Product) -> String {
    //     match product.specs.get("chipset").and_then(|c| c.as_str()) {
    //         Some(chipset) => match product.specs.get("memory_size").and_then(|s| s.as_str()) {
    //             Some(size) => format!("{chipset} {size}GB"),
    //             None => chipset.to_string()
    //         },
    //         None => product.name.clone()
    //     }
    // }

    fn extract_from_dataset(
        &self,
        text: &str,
        dataset: &Vec<DatasetEntry>,
        specs: &mut Value,
        remove_optionals: bool
    ) -> Option<(String, Value)> {
        let text = match remove_optionals {
            true => self.remove_optional_words(text).to_uppercase(),
            false => text.to_uppercase()
        };

        let memory_size = specs.get("memory_size")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        for entry in dataset {
            let entry_name = match remove_optionals {
                true => self.remove_optional_words(&entry.name).to_uppercase(),
                false => entry.name.to_uppercase()
            };

            let mut matched = true;
            for word in entry_name.split_whitespace() {
                if !(text.contains(word) || (word.len() <= 4 && word.ends_with("GB"))) {
                    matched = false;
                    break;
                }
            }

            if matched && let Some(chipset) = &entry.chipset {
                let chipset_size = chipset.data.get("memory_size")
                    .and_then(|s| s.as_str())
                    .map(|s| s.to_string());
                if chipset_size == memory_size || memory_size.is_none() {
                    specs["chipset"] = Value::String(chipset.name.clone());
                    return Some((entry.name.clone(), chipset.data.clone()));
                }
            }
        }

        if remove_optionals {
            for chipset in &self.chipsets {
                let chipset_name = &chipset.name;
                let entry_name = self.remove_optional_words(chipset_name).to_uppercase();

                let mut matched = true;
                for word in entry_name.split_whitespace() {
                    if !text.contains(word) {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    let chipset_size = chipset.data.get("memory_size")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string());
                    if chipset_size == memory_size || memory_size.is_none() {
                        specs["chipset"] = Value::String(chipset_name.clone());
                        let name = match (specs.get("brand").and_then(|b| b.as_str()), chipset_size) {
                            (Some(brand), Some(size)) => format!("{brand} {chipset_name} {size}GB"),
                            (Some(brand), None) => format!("{brand} {chipset_name}"),
                            (None, Some(size)) => format!("{chipset_name} {size}GB"),
                            (None, None) => chipset_name.clone(),
                        };
                        return Some((name, chipset.data.clone()));
                    }
                }
            }
        }

        None
    }
}