pub mod gpu_parser;
pub mod memory_parser;
pub mod storage_parser;
pub mod pc_parser;

use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::errors::ParseErrorKind;
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::{ChipsetEntry, DatasetEntry, Section, SectionConfig};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

static COLORS: &[&[&str]] = &[
    &["Blanc", "White"], &["Noir", "Black"], &["Bleu", "Blue"], &["Rouge", "Red"], &["Jeune", "Yellow"],
    &["Verte", "Green"], &["Gris", "Gray"]
];

pub trait SectionParser: Send + Sync {
    fn config(&self) -> Arc<SectionConfig>;
    
    fn dataset(&self) -> &Vec<DatasetEntry>;
    
    fn chipsets(&self) -> &[ChipsetEntry] {
        &[]
    }

    fn matches(&self, title: &str, desc: &Option<String>, skip_include_check: bool) -> bool {
        let config = self.config();

        for include in &config.force_include {
            if RegexCache::custom_match(include, title) {
                return true;
            }
        }

        if !skip_include_check && !self.matches_include(title) {
            match desc {
                None => return false,
                Some(desc) => {
                    let text = format!("{title} {desc}");
                    if !config.include_description.iter().any(|include| RegexCache::custom_match(include, &text)) {
                        return false
                    }
                }
            }
        }

        if config.exclude.iter().any(|exclude| RegexCache::custom_match(exclude, title)) {
            return false
        }

        if let Some(desc) = desc {
            let text = format!("{title} {desc}");
            if config.exclude_description.iter().any(|exclude| RegexCache::custom_match(exclude, &text)) {
                return false
            }
        }

        true
    }
    
    fn matches_include(&self, text: &str) -> bool {
        self.config().include.iter().any(|include| RegexCache::custom_match(include, text))
    }

    fn parse(&self, product: &mut Product) -> Result<(), ParseErrorKind> {
        product.name = self.clean_title(&product.title);
        self.parse_specs(product);
        self.parse_brand(product);

        match self.lookup_dataset(product) {
            Some(name) => {
                product.name = name;
                self.post_processing(product);
                Ok(())
            },
            None => Err(ParseErrorKind::NotInDataset)
        }
    }

    fn parse_specs(&self, _product: &mut Product) {
    }

    fn parse_brand(&self, product: &mut Product) {
        for brand in &self.config().brands {
            if RegexCache::matches(&format!("(?i)\\b{brand}\\b"), &product.name) {
                product.specs["brand"] = Value::String(brand.clone());
                return
            }
        }
        //WARNING: No brand detected
    }

    fn clean_title(&self, title: &str) -> String {
        let cleaner = &self.config().title_cleaner;
        let mut text = title.replace("®", "").replace("™", "");

        // Replace Colors
        // for color in COLORS {
        //     if let (Some(from), Some(to)) = (color.first(), color.get(1)) {
        //         let pattern = format!("(?i)\\b{}\\b", regex::escape(from));
        //         text = RegexCache::replace_all(&pattern, &text, to).to_string();
        //     }
        // }

        // Remove Words
        for word in &cleaner.remove_words {
            let pattern = format!("(?i)\\b{}\\b", regex::escape(word));
            text = RegexCache::replace_all(&pattern, &text, "").to_string();
        }

        // Remove Patterns
        for pattern in &cleaner.remove_patterns {
            text = RegexCache::replace_all(pattern, &text, "").to_string();
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

    fn post_processing(&self, _product: &mut Product) {
    }

    fn lookup_dataset(&self, product: &mut Product) -> Option<String> {
        let dataset = self.dataset();
        if dataset.is_empty() {
            return Some(product.name.clone())
        }

        let specs = &mut product.specs;
        let text = match &product.description {
            Some(desc) => format!("{} {desc}", product.name),
            None => product.name.clone()
        };

        let result = match self.extract_from_dataset(&text, dataset, specs, false) {
            Some(res) => Some(res),
            None => self.extract_from_dataset(&text, dataset, specs, true)
        };

        if let Some((name, entry)) = result {
            for (key, value) in entry.as_object().unwrap() {
                if key == "name" {
                    continue;
                }
                specs[key] = value.clone();
            }
            return Some(name)
        }

        None
    }

    fn extract_from_dataset(
        &self,
        text: &str,
        dataset: &Vec<DatasetEntry>,
        _specs: &mut Value,
        remove_optionals: bool
    ) -> Option<(String, Value)> {
        let text = match remove_optionals {
            true => self.remove_optional_words(text).to_uppercase(),
            false => text.to_uppercase()
        };

        for entry in dataset {
            let entry_name = match remove_optionals {
                true => self.remove_optional_words(&entry.name).to_uppercase(),
                false => entry.name.to_uppercase()
            };
            
            let mut matched = true;
            // To avoid false checks with memory
            if entry_name.ends_with(" 16") {
                if !text.contains(&entry_name) {
                    matched = false;
                }
            } else {
                for word in entry_name.split_whitespace() {
                    if !text.contains(word) {
                        matched = false;
                        break;
                    }
                }
            }

            if matched {
                return Some((entry.name.clone(), entry.data.clone()));
            }
        }

        None
    }

    fn remove_optional_words(&self, text: &str) -> String {
        let mut text = text.to_string();

        let words: Vec<String> = self.config()
            .optional_dataset_words
            .iter()
            .map(|w| regex::escape(w))
            .collect();

        if !words.is_empty() {
            let pattern = format!(r"(?i)(?:{})", words.join("|"));
            text = RegexCache::replace_all(&pattern, &text, "").to_string();

            return text.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
        }

        text
    }
}

pub struct GenericSectionParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Vec<DatasetEntry>
}

impl SectionParser for GenericSectionParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Vec<DatasetEntry> {
        &self.dataset
    }
}