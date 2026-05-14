pub mod pc_parser;
pub mod component_parser;
mod patterns;

use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::errors::ParseErrorKind;
use crate::web_scraper::sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionConfig {
    pub id: String,
    pub use_dataset: bool,
    pub keywords: Vec<String>,
    pub exclude: Vec<String>,
    pub subsections: Vec<SubSectionConfig>,
    pub required_fields: Vec<String>,
    pub title_cleaner: TitleCleanerConfig,
    pub specs: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubSectionConfig {
    pub id: String,
    pub detectable: bool,
    pub keywords: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleCleanerConfig {
    replace_words: Vec<Vec<String>>,
    remove_words: Vec<String>,
    remove_patterns: Vec<String>,
}

pub trait SectionParser: Send + Sync {
    fn config(&self) -> &SectionConfig;
    fn dataset(&self) -> &Option<Vec<Value>>;

    fn matches_keywords(&self, text: &str) -> bool {
        let config = self.config();
        config.keywords.iter().any(|pattern| RegexCache::matches(pattern, text)) &&
            !config.exclude.iter().any(|pattern| RegexCache::matches(pattern, text))
    }

    fn detect_subsection(&self, text: &str) -> Option<Section> {
        for rule in &self.config().subsections {
            if rule.detectable {
                if !rule.keywords.iter().any(|pattern| RegexCache::matches(pattern, text)) {
                    continue;
                }

                if !rule.exclude.iter().any(|pattern| RegexCache::matches(pattern, text)) {
                    return Some(Section::from_str(&rule.id).unwrap());
                }
            }
        }
        None
    }

    fn parse_specs(&self, specs: &mut Value, name: &str, _desc: &Option<String>) {
        for (field_name, regex) in &self.config().specs {
            if let Some(caps) = RegexCache::captures(&regex, name) {
                if let Some(m) = caps.get(1) {
                    specs[field_name] = json!(m.as_str().to_uppercase());
                    continue;
                }
            }
        }
    }

    fn validate_required_fields(&self, specs: &Value) -> Result<(), ParseErrorKind> {
        for field in &self.config().required_fields {
            if specs.get(field).is_none() || specs[field].is_null() {
                return Err(ParseErrorKind::IncompleteData(format!("Missing required field: `{}`", field)))
            }
        }
        Ok(())
    }

    fn clean_title(&self, title: &str) -> String {
        let cleaner = &self.config().title_cleaner;
        let mut text = title.to_string();

        // Replace words
        for list in &cleaner.replace_words {
            if let (Some(from), Some(to)) = (list.first(), list.get(1)) {
                let pattern = format!("(?i)\\b{}\\b", regex::escape(from));
                text = RegexCache::replace_all(&pattern, &text, to).to_string();
            }
        }

        // Remove words
        for word in &cleaner.remove_words {
            let pattern = format!("(?i)\\b{}\\b", regex::escape(word));
            text = RegexCache::replace_all(&pattern, &text, "").to_string();
        }

        // Remove patterns
        for pattern in &cleaner.remove_patterns {
            text = RegexCache::replace_all(pattern, &text, "").to_string();
        }

        // Normalize whitespace
        text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        text.trim().to_string()
    }

    fn lookup_dataset(&self, name: &str, specs: &mut Value) -> Option<String> {
        // If Section does not have a dataset return name as the id
        let Some(dataset) = self.dataset().as_ref() else {
            return Some(name.to_string())
        };

        if let Some((id, dataset_entry)) = self.extract_from_dataset(name, dataset) {
            self.parse_specs_from_dataset(specs, &dataset_entry);
            return Some(id)
        }

        None
    }

    fn extract_from_dataset(&self, name: &str, dataset: &[Value]) -> Option<(String, Value)> {
        let lower = name.to_lowercase();
        if lower.is_empty() {
            return None;
        }

        for entry in dataset {
            let Some(id) = entry.get("id").and_then(|n| n.as_str()) else { continue };

            // Check id
            let pattern = format!(r"\b{}\b", regex::escape(&id.to_lowercase()));
            if RegexCache::matches(&pattern, &lower) {
                return Some((id.to_string(), entry.clone()));
            }

            // Check keywords
            if let Some(kws) = entry.get("keywords").and_then(|k| k.as_array()) {
                for kw in kws {
                    if let Some(kw_str) = kw.as_str() {
                        let pattern = format!(r"\b{}\b", regex::escape(&kw_str.to_lowercase()));
                        if RegexCache::matches(&pattern, &lower) {
                            return Some((id.to_string(), entry.clone()));
                        }
                    }
                }
            }
        }

        None
    }

    fn parse_specs_from_dataset(&self, specs: &mut Value, entry: &Value) {
        for (key, value) in entry.as_object().unwrap() {
            // Skip metadata fields
            if key == "id" || key == "keywords" {
                continue;
            }
            specs[key] = value.clone();
        }
    }
}

pub struct GenericSectionParser {
    pub config: SectionConfig,
    pub dataset: Option<Vec<Value>>
}

impl SectionParser for GenericSectionParser {
    fn config(&self) -> &SectionConfig {
        &self.config
    }

    fn dataset(&self) -> &Option<Vec<Value>> {
        &self.dataset
    }
}