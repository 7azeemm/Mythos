use crate::utils::regex_cache::RegexCache;
use crate::utils::serde_ext::JsonExt;
use crate::web_scraper::dataset::{Dataset, FilterNode, SearchResult};
use crate::web_scraper::parsers::monitor_parser::extract_display_specs;
use crate::web_scraper::parsers::{words_match, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use serde_json::Value;
use std::sync::Arc;

static KNOWN_SIZES: &[i32] = &[
    1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 20, 24, 32, 40, 48, 64, 96, 120, 128, 240, 250, 256, 265, 320,
    480, 500, 512, 640, 960, 1000, 1024, 1512, 2048, 3072, 4096
];

pub struct PCParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Dataset
}

impl SectionParser for PCParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product, cleaned_title: &str) {
        let desc = product.description.clone().unwrap_or_default();
        let text: String = format!("{} | {desc}", cleaned_title).to_uppercase();
        let text = product.section.config().title_cleaner.replace_words(&text, true);
        let text = text.replace(" .6\"", ".6\"").replace(".,", ".").replace("GRAPHIQUE", "GRAPHICS")
            .replace("GRAPHIC ", "GRAPHICS ").replace("ᵉ", "E").replace("‑", "-").replace("™", "")
            .replace("®", "").replace("(TM)", "").replace("–", "").replace("PROCESSOR ", "");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

        let cpu = extract_cpu(&text);
        let gpu = extract_gpu(self.config.id, &text, &cpu.as_ref().map(|s| s.data.clone()).flatten());

        let gpu_has_memory = gpu.as_ref()
            .map(|s| s.data.get_str("memory_size").map(|s| !s.is_empty()))
            .flatten().unwrap_or(false);

        let mut memory: Option<i32> = None;
        let mut storage: Option<i32> = None;
        let mut gpu_memory: Option<i32> = None;

        // Extract Sizes from Title and Description
        let mut desc_sizes = get_sizes(desc.clone(), false);
        let title_sizes = get_sizes(product.title.clone(), true);
        let (storage_sizes, mut title_sizes): (Vec<_>, Vec<_>) = title_sizes.into_iter()
            .partition(|(size, unit)| unit == "TB" || *size > 128 || (*size == 128 && product.title.contains("+128")));
        title_sizes.sort_by_key(|(size, _)| *size);

        // Find Storage Size in Title (Additive)
        for (mut size, unit) in storage_sizes {
            if unit == "TB" {
                size = size * 1024;
            }
            storage = Some(match storage {
                Some(old_size) => old_size + size,
                None => size
            })
        }

        // Find Storage Size in Description if not found
        if storage.is_none() {
            for (size, unit) in desc_sizes.iter() {
                let mut size = *size;
                if unit == "TB" || size > 128 || (size == 128 && product.price < 2000) {
                    if unit == "TB" {
                        size = size * 1024;
                    }
                    storage = Some(size);
                    break;
                }
            }
        }

        if storage.is_none() && text.contains("512") {
            storage = Some(512);
        }

        // Find Sizes in title (Memory, Storage, GPU Memory)
        match (title_sizes.first(), title_sizes.get(1)) {
            (Some((size, _)), None) if *size > 128 && storage.is_none() => storage = Some(*size),
            (Some((size, _)), None) => memory = Some(*size),
            (Some((size, _)), Some((size_2, _))) => {
                if storage.is_none() && *size_2 >= 32 {
                    memory = Some(*size);
                    storage = Some(*size_2);
                } else if gpu_has_memory {
                    gpu_memory = Some(*size);
                    memory = Some(*size_2);
                } else if *size_2 <= 32 {
                    memory = Some(*size_2);
                }
            },
            (None, _) => {}
        }

        // Find GPU Memory
        if gpu_has_memory && gpu_memory.is_none() {
            let text = desc.to_uppercase().replace("AVEC ", "");
            let pattern = r"(?i)(\d+)\s*(?:g|go|gb)\s+(?:(?:de\s+)?m[eé]moire\s+d[eé]di[eé]e|(?:gddr[0-9]|gddr\s*[0-9]))";

            // Using Pattern
            if let Some(caps) = RegexCache::captures(&pattern, &text) {
                if let Some(size) = caps.get(1).and_then(|v| v.as_str().parse::<i32>().ok()) {
                    gpu_memory = Some(size);
                }
            }
        }

        // Find RAM and GPU Memory if not found from description
        if gpu_memory.is_none() || memory.is_none() {
            let mut sizes = vec![];
            let mut removed = false;

            for (size, unit) in &desc_sizes {
                if unit == "GB" && *size <= 64 {
                    if !removed && let Some(memory) = &memory {
                        if size == memory {
                            removed = true;
                            continue
                        }
                    }

                    if !removed && let Some(gpu_memory) = &gpu_memory {
                        if size == gpu_memory {
                            removed = true;
                            continue
                        }
                    }

                    sizes.push(*size);
                }
            }

            sizes.sort();

            match (sizes.first(), sizes.get(1)) {
                (Some(size), Some(size_2)) => {
                    gpu_memory = Some(*size);
                    memory = Some(*size_2);
                },
                (Some(size), _) => {
                    if memory.is_none() {
                        memory = Some(*size);
                    } else if gpu_has_memory && gpu_memory.is_none() {
                        gpu_memory = Some(*size);
                    }
                }
                (None, _) => {}
            }
        }

        // Extract Display
        if vec![Section::Laptop, Section::GamingLaptop, Section::MacBook, Section::AllInOnePC].contains(&product.section) {
            extract_display_specs(product, &text, product.section != Section::AllInOnePC);
        }

        if let Some(cpu) = cpu {
            product.filter_ids.insert("cpu".to_string(), cpu.id);
            product.components.insert("cpu".to_string(), cpu.label);
        }

        if let Some(gpu) = gpu {
            product.filter_ids.insert("gpu".to_string(), gpu.id);
            if let Some(memory) = gpu_memory {
                let memory = format!("{memory}GB");
                product.components.insert("gpu".to_string(), format!("{} {memory}", gpu.label));
                product.filter_ids.insert("gpu_memory".to_string(), memory);
            } else {
                product.components.insert("gpu".to_string(), gpu.label);
            }
        }

        if let Some(memory) = memory {
            let memory_size = format!("{memory}GB");
            product.filter_ids.insert("memory_size".to_string(), memory_size.clone());

            let mut found_type = false;
            for memory_type in vec!["DDR3", "DDR4", "DDR5"] {
                if text.contains(memory_type) {
                    product.filter_ids.insert("memory_type".to_string(), memory_type.to_string());
                    product.components.insert("memory".to_string(), format!("{memory_size} {memory_type}"));
                    found_type = true;
                    break;
                }
            }

            if !found_type {
                product.components.insert("memory".to_string(), memory_size);
            }
        }

        if let Some(storage) = storage {
            let storage_size = match storage >= 1000 {
                true => format!("{:.1}TB", storage as f64 / 1024.0).replace(".0T", "T"),
                false => format!("{storage}GB")
            };

            product.filter_ids.insert("storage_size".to_string(), storage_size.clone());
            product.components.insert("storage".to_string(), if text.contains("NVME") {
                format!("{storage_size} NVME")
            } else if text.contains("SSD") {
                format!("{storage_size} SSD")
            } else if text.contains("HDD") {
                format!("{storage_size} HDD")
            } else {
                storage_size
            });
        }
    }
}

fn extract_cpu(text: &str) -> Option<SearchResult> {
    let section = Section::CPU;
    let text = section.config().title_cleaner.replace_words(&text, true);
    let optionals = &["AMD", "Intel", "Core", "Gold", "Silver", "Processor", "Gemini Lake", "Jasper Lake"];

    // Dataset Search
    for node in &section.parser().dataset().nodes {
        // Skip if cpu is AMD and product desc contains a reference to an Intel cpu
        if node.label.contains("AMD") && text.contains("INTEL CORE") {
            continue
        }

        if words_match(&text, &node.label.to_uppercase(), optionals) {
            return Some(SearchResult {
                id: node.id.clone(),
                label: node.label.clone(),
                data: node.data.clone()
            })
        }
    }

    // Search for Intel CPUS
    for cpu in vec!["i3", "i5", "i7", "i9", "Ultra 5", "Ultra 7", "Ultra 9", "Core 3", "Core 5", "Core 7", "Core 9"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            let mut label = match cpu.contains("Core") {
                true => format!("Intel {cpu}"),
                false => format!("Intel Core {cpu}")
            };

            if label.contains("i") {
                let generation_pattern = r"(?i)\b([2-9]|1[0-4])\s*(?:th|é|e|è|éme|ème|eme|gén|gen)\s*(?:gen|gén|generation|génération)?\b";
                if let Some(caps) = RegexCache::captures(&generation_pattern, &text) {
                    if let Some(generation) = caps.get(1).and_then(|v| v.as_str().parse::<i32>().ok()) {
                        label.push_str(&format!(" {generation}th Gen"));
                    }
                }
            }

            return Some(SearchResult {
                id: "Others/Intel".to_string(),
                label,
                data: None
            });
        }
    }

    // Search for AMD CPUS
    for cpu in vec!["Ryzen 3", "Ryzen 5", "Ryzen 7", "Ryzen 9", "Ryzen"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            return Some(SearchResult {
                id: "Others/AMD".to_string(),
                label: format!("AMD {cpu}"),
                data: None
            })
        }
    }

    // Search for Other CPUS
    for cpu in vec!["Quad Core", "Quad-Core", "Dual Core", "Dual-Core", "Arm"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            let cpu = cpu.replace("-", " ");
            let (id, label) = if text.contains("INTEL") {
                ("Others/Intel", format!("Intel {cpu} Processor"))
            } else if text.contains("AMD") {
                ("Others/AMD", format!("AMD {cpu} Processor"))
            } else {
                ("Others/Arm", format!("{cpu} Processor"))
            };

            return Some(SearchResult {
                id: id.to_string(),
                label,
                data: None
            })
        }
    }

    None
}

fn extract_gpu(id: Section, text: &str, cpu_entry: &Option<Value>) -> Option<SearchResult> {
    let section = Section::GPU;
    let text = section.config().title_cleaner.replace_words(&text, true);
    let has_dedicated_gpu = vec!["RTX", "RX", "GTX", " GT "].into_iter().any(|w| text.contains(w));
    let optionals = &["GeForce", "Radeon", "Nvidia", "Intel", "Graphics"];

    // Dataset Search
    for node in &section.parser().dataset().nodes {
        // Skip if: gpu is a vendor card
        // or product is laptop and chip is not mobile compatible
        // or chip is IGPU and product has dGPU
        if node.data.get_bool("vendor_card") == Some(true) ||
            (id.is_laptop() && node.data.get_bool("laptop_support") != Some(true)) ||
            (has_dedicated_gpu && node.label.contains("Graphics")) {
            continue
        }

        if words_match(&text, &node.label.to_uppercase(), optionals) {
            let label = node.label.split_whitespace()
                .filter(|w| !(w.len() <= 4 && w.contains("GB")))
                .collect::<Vec<_>>().join(" ");
            return Some(SearchResult {
                id: node.id.clone(),
                label,
                data: node.data.clone(),
            })
        }
    }

    if has_dedicated_gpu {
        return None
    }

    if id != Section::GamingLaptop {
        // Extract IGPU from CPU Entry
        if let Some(entry) = &cpu_entry {
            if let Some(iGPU) = entry.get_str("integrated_gpu") {
                if !iGPU.is_empty() {
                    let id = if iGPU.contains("UHD") {
                        format!("Intel/Intel UHD Graphics/{iGPU}")
                    } else if iGPU.contains("Radeon") {
                        format!("AMD/{iGPU}")
                    } else {
                        eprintln!("Unknown Integrated GPU: {iGPU}");
                        "Others".to_string()
                    };
                    return Some(SearchResult {
                        id,
                        label: iGPU.to_string(),
                        data: None
                    })
                }
            }
        }
    }

    // Fallback checks
    let gpu = if text.contains("INTEL ") {
        if text.contains(" ARC ") {
            Some(("Arc/Intel Arc Graphics", "Intel Arc Graphics"))
        } else if text.contains("UHD GRAPHICS") {
            Some(("Intel/Intel UHD Graphics/Intel UHD Graphics", "Intel UHD Graphics"))
        } else if text.contains("HD GRAPHICS") {
            Some(("Intel/Intel HD Graphics/Intel HD Graphics", "Intel HD Graphics"))
        } else if text.contains("GRAPHICS") {
            Some(("Intel/Intel Graphics", "Intel Graphics"))
        } else {
            None
        }
    } else if text.contains("AMD RADEON") {
        Some(("AMD/AMD Radeon Graphics", "AMD Radeon Graphics"))
    } else {
        None
    };

    if let Some((id, label)) = gpu {
        return Some(SearchResult {
            id: id.to_string(),
            label: label.to_string(),
            data: None
        })
    }

    None
}

fn get_sizes(text: String, title: bool) -> Vec<(i32, String)> {
    let mut sizes = Vec::new();

    let size_pattern = if title {
        r"(?i)(\d+)\s*(?:g|go|gb|to|tb|t|tera|ssd)\b"
    } else {
        r"(?i)(\d+)\s*(?:g|go|gb|to|tb|tera|t o)\b"
    };

    for caps in RegexCache::captures_iter(size_pattern, &text) {
        let Some(num) = caps.get(1).and_then(|v| v.as_str().parse::<i32>().ok()) else {
            continue;
        };

        if num > 5000 || !KNOWN_SIZES.contains(&num) {
            continue;
        }

        let match_pos = caps.get(0).unwrap();
        let start = match_pos.start();

        let mut unit = match_pos.as_str().chars().rev()
            .take_while(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .chars().rev().collect::<String>().to_uppercase();

        if start > 1 {
            let mut before_iter = text[..start].chars().rev().take(2);
            if let (Some(last_char), Some(before_last_char)) = (before_iter.next(), before_iter.next()) {
                // Skip if last char is an alphabetic and not 'e' or ' O'
                if last_char.is_alphabetic() {
                    if last_char != 'e' && !(last_char == 'O' && before_last_char == ' ') {
                        if unit != "GO" && unit != "TO" {
                            continue;
                        }
                    }
                }

                // Skip "(2 x 16Go)"
                let mut after_iter = text[match_pos.end()..].chars().take(2);
                let contains_paren = after_iter.next() == Some(')') || after_iter.next() == Some(')');
                let before_last_char = before_last_char.to_ascii_lowercase();
                if last_char == ' ' && before_last_char == 'x' && contains_paren {
                    continue;
                }
            }
        }

        if unit == "T" && num >= 4 {
            continue
        }

        unit = match unit.trim() {
            "TB" | "TO" | "T" | "T O" | "TERA" => "TB".to_string(),
            "GB" | "GO" | "G" | "SSD" => "GB".to_string(),
            s => s.to_string(),
        };

        if unit == "TB" && num > 8 {
            continue
        }

        sizes.push((num, unit));
    }

    sizes
}