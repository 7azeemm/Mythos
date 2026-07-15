pub mod gpu_parser;
pub mod memory_parser;
pub mod storage_parser;
pub mod pc_parser;
pub mod monitor_parser;

use crate::utils::regex_cache::RegexCache;
use crate::utils::serde_ext::JsonExt;
use crate::web_scraper::dataset::{Dataset, SearchResult};
use crate::web_scraper::errors::ParseErrorKind;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::{Section, SectionConfig};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

//TODO: reset button does not register a send_section_info request

pub trait SectionParser: Send + Sync {
    fn config(&self) -> Arc<SectionConfig>;
    
    fn dataset(&self) -> &Dataset;
    
    fn parse(&self, product: &mut Product) -> Result<(), ParseErrorKind> {
        let cleaned_title = self.clean_title(&product.title);
        self.parse_specs(product, &cleaned_title);
        self.parse_brand(product, &cleaned_title);

        let name = self.lookup_dataset(product, &cleaned_title);
        let details = self.post_processing(product);

        match name {
            Some(name) => {
                let value = details.map(|text| format!("{name} {text}")).unwrap_or(name);
                product.components.insert(self.config().id_field_name.clone(), value);
                Ok(())
            },
            None => Err(ParseErrorKind::NotInDataset)
        }
    }

    fn parse_specs(&self, _product: &mut Product, _text: &str) {
    }

    fn parse_brand(&self, product: &mut Product, text: &str) {
        for brand in &self.config().brands {
            if RegexCache::matches(&format!("(?i)\\b{brand}\\b"), text) {
                // product.specs.set("brand", brand.as_str());
                return
            }
        }
        //WARNING: No brand detected
    }

    fn clean_title(&self, title: &str) -> String {
        let cleaner = &self.config().title_cleaner;
        let mut text = title.replace("®", "").replace("™", "");

        // Remove Words
        for word in &cleaner.remove_words {
            let pattern = format!("(?i)\\b{}\\b", regex::escape(word));
            text = RegexCache::replace_all(&pattern, &text, "").to_string();
        }

        // Replace Words
        for list in &cleaner.replace_words {
            if let (Some(from), Some(to)) = (list.first(), list.get(1)) {
                let pattern = format!("(?i){}", regex::escape(from));
                text = RegexCache::replace_all(&pattern, &text, to).to_string();
            }
        }

        // Add Missing Brands
        let lower = text.to_lowercase();
        for list in &cleaner.add_brands_by_models {
            if let (Some(brand), Some(word)) = (list.first(), list.get(1)) {
                if !lower.contains(&brand.to_lowercase()) && lower.contains(&word.to_lowercase()) {
                    text = format!("{brand} {text}");
                }
            }
        }

        // Normalize whitespace and remove duplicates
        let mut seen = HashSet::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let deduplicated: Vec<&str> = words.into_iter()
            .filter(|word| seen.insert(word.to_lowercase()))
            .collect();

        deduplicated.join(" ").trim().to_string()
    }

    fn post_processing(&self, _product: &mut Product) -> Option<String> {
        None
    }

    fn lookup_dataset(&self, product: &mut Product, title: &str) -> Option<String> {
        if self.dataset().nodes.is_empty() {
            return Some(title.to_string())
        }

        let text = product.description.clone()
            .map(|desc| format!("{title} {desc}"))
            .unwrap_or(title.to_string())
            .to_uppercase();

        let memory_size = match product.section {
            Section::GPU => product.filter_ids.get("memory_size").map(String::as_str),
            _ => None
        };

        if let Some(result) = self.search_in_dataset(&text, memory_size) {
            product.filter_ids.insert(self.config().id_field_name.clone(), result.id);
            if let Some(Value::Object(obj)) = result.data {
                for (key, value) in obj {
                    if let Some(value) = value.as_str() && !value.is_empty() {
                        product.filter_ids.insert(key, value.to_string());
                    }
                }
            }
            return Some(result.label)
        }

        None
    }

    fn search_in_dataset(&self, text: &str, memory_size: Option<&str>) -> Option<SearchResult> {
        for node in &self.dataset().nodes {
            if !words_match(&text, &node.label.to_uppercase(), &node.optional_words) {
                continue;
            }

            if self.config().id != Section::GPU {
                return Some(SearchResult {
                    id: node.id.clone(),
                    label: node.label.clone(),
                    data: node.data.clone()
                })
            }

            let Some(chipset_data) = &node.data else { continue };
            let chipset_size = chipset_data.get_str("memory_size");
            if chipset_size == memory_size || memory_size.is_none() {
                return Some(SearchResult {
                    id: node.id.clone(),
                    label: node.label.clone(),
                    data: Some(chipset_data.clone())
                })
            }
        }
        
        None
    }
}

pub struct GenericSectionParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for GenericSectionParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}

pub fn words_match<T: AsRef<str>>(text: &str, candidate: &str, optionals: &[T]) -> bool {
    candidate.split_whitespace().all(|word| {
        optionals.iter().any(|o| o.as_ref().eq_ignore_ascii_case(word))
            || word.len() <= 4 && word.ends_with("GB")
            || RegexCache::matches(&format!("\\b{word}\\b"), text)
    })
}