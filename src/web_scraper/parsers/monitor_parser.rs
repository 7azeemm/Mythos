use std::sync::Arc;
use serde_json::to_string;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::SectionParser;
use crate::web_scraper::product::{Product, Specs};
use crate::web_scraper::sections::{SectionConfig};

pub struct MonitorParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for MonitorParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, cleaned_title: &str) {
        let desc = product.description.clone().unwrap_or_default();
        let text: String = format!("{cleaned_title} | {desc}").to_uppercase();
        extract_display_specs(product, &text, false);
    }
}

pub fn extract_display_specs(product: &mut Product, text: &str, laptop: bool) {
    let mut specs_list = Vec::new();

    let mut found_size = false;
    let size_pattern = "(?i)\\b(\\d+(?:[.,]\\d+)?)\\s*(?:″|”|′|'|\"| »|inch|pouce)";
    for caps in RegexCache::captures_iter(size_pattern, text) {
        if let Some(size_str) = caps.get(1).map(|v| v.as_str()) {
            let size_str = size_str.replace(",", ".");
            if let Ok(size) = size_str.parse::<f32>() {
                // filter impossible screen sizes
                let range = if laptop { 10.0..=18.0 } else { 10.0..=120.0 };
                if !range.contains(&size) {
                    continue;
                }

                let size = format!("{}\"", size_str.replace(".0", ""));
                product.filter_ids.insert("display_size".to_string(), size.clone());
                specs_list.push(size);
                found_size = true;
                break;
            }
        }
    }

    if !found_size {
        if text.replace(",", ".").contains("15.6") {
            let size = "15.6".to_string();
            product.filter_ids.insert("display_size".to_string(), size.clone());
            specs_list.push(size);
        }
    }

    let mut found_resolution = false;
    let resolution_pattern = r"(?i)\b(FULL\s*HD|FHD|QHD|UHD|HD|WFHD|UWQHD|UWFHD|2K|2[.,]5K|2[.,]8K|3K|4K|5K|WUXGA|WQXGA|WXGA|WQHD)\b";
    for caps in RegexCache::captures_iter(resolution_pattern, text) {
        if let Some(mat) = caps.get(0) {
            let mut resolution = mat.as_str().trim().to_uppercase()
                .replace(",", ".")
                .replace("FULL HD", "FHD")
                .replace("FULLHD", "FHD");

            if resolution == "HD" || resolution == "UHD" {
                let str = format!("{resolution} GRAPHICS");
                if text.contains(&format!(" {str}")) {
                    if let Some(pos) = text.find(&str) {
                        if pos == mat.start() {
                            continue
                        }
                    }
                }
            }

            if text.as_bytes().get(mat.end()) == Some(&b'+') {
                resolution.push('+');
            }

            product.filter_ids.insert("resolution".to_string(), resolution.clone());
            specs_list.push(resolution);
            found_resolution = true;
            break;
        }
    };

    if !found_resolution {
        let resolutions = vec![
            ("1080", "1920", "FHD".to_string()),
            ("900", "1600", "HD+".to_string()),
            ("1440", "2560", "QHD".to_string())
        ];
        for (num1, num2, resolution) in resolutions {
            if text.contains(num1) && text.contains(num2) {
                product.filter_ids.insert("resolution".to_string(), resolution.clone());
                specs_list.push(resolution);
                break;
            }
        }
    }

    let panel_pattern = r"(?i)\b(IPS|OLED|LCD)\b";
    if let Some(caps) = RegexCache::captures(panel_pattern, text) {
        if let Some(panel_type) = caps.get(1).and_then(|v| Some(v.as_str().to_string())) {
            product.filter_ids.insert("panel_type".to_string(), panel_type.clone());
            specs_list.push(panel_type);
        }
    };

    let refresh_pattern = r"(?i)\b(\d+)\s*(hz|h z |hertz)\b";
    let mut refresh_rate: Option<i32> = None;
    for caps in RegexCache::captures_iter(refresh_pattern, text) {
        if let Some(rate) = caps.get(1).and_then(|m| m.as_str().parse::<i32>().ok()) {
            if rate >= 60 {
                refresh_rate = Some(refresh_rate.map_or(rate, |b| b.max(rate)));
            }
        }
    }
    if let Some(refresh_rate) = refresh_rate {
        let refresh_rate = format!("{refresh_rate}Hz");
        product.filter_ids.insert("refresh_rate".to_string(), refresh_rate.clone());
        specs_list.push(refresh_rate);
    }

    if !specs_list.is_empty() {
        product.components.insert("display".to_string(), specs_list.join(" "));
    }
}