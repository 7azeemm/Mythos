use crate::utils::regex_cache::RegexCache;
use crate::utils::str_utils::remove_words;
use crate::web_scraper::parsers::{DatasetEntry, SectionConfig, SectionParser};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::{ChipsetEntry, Section};
use serde_json::Value;
use std::sync::Arc;

static KNOWN_SIZES: &[i32] = &[
    1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 16, 18, 20, 24, 32, 40, 48, 64, 96, 120, 128, 240, 250, 256, 265, 320,
    480, 500, 512, 640, 960, 1000, 1024, 1512, 2048, 3072, 4096
];

pub struct PCParser {
    pub config: Arc<SectionConfig>,
    pub dataset: Vec<DatasetEntry>
}

impl SectionParser for PCParser {
    fn config(&self) -> Arc<SectionConfig> {
        self.config.clone()
    }

    fn dataset(&self) -> &Vec<DatasetEntry> {
        &self.dataset
    }

    fn parse_specs(&self, product: &mut Product) {
        let desc = product.description.clone().unwrap_or_default();
        let text: String = format!("{} | {desc}", product.title).to_uppercase()
            .replace("GRAPHIQUE", "GRAPHICS")
            .replace("GRAPHIC ", "GRAPHICS ")
            .replace("ᵉ", "E")
            .replace("‑", "-");
        let text = remove_words(&text, &["™", "®", "(TM)", "–", "PROCESSOR "]);
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

        let (cpu, cpu_entry) = extract_cpu(&text);
        let (gpu, gpu_chipset) = extract_gpu(&text, &cpu_entry);

        let gpu_has_memory = gpu_chipset.as_ref()
            .map(|g| g.data.get("memory_size")
                .and_then(|s| s.as_str()).map(|s| !s.is_empty()).unwrap_or(false)
            ).unwrap_or(false);

        let mut memory: Option<i32> = None;
        let mut storage: Option<i32> = None;
        let mut gpu_memory: Option<i32> = None;

        // Extract Sizes from Title and Description
        let mut desc_sizes = get_sizes(desc.clone(), false);
        let title_sizes = get_sizes(product.title.clone(), true);
        let (storage_sizes, mut title_sizes): (Vec<_>, Vec<_>) = title_sizes.into_iter()
            .partition(|(size, unit)| unit == "TB" || *size > 128 || (*size == 128 && product.title.contains("+128")));
        title_sizes.sort_by_key(|(size, _)| *size);

        // Find Storage Size in Title (Addictive)
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

        let specs = &mut product.specs;

        // Extract Display
        if vec![Section::Laptop, Section::GamingLaptop, Section::MacBook, Section::AllInOnePC].contains(&product.section) {
            extract_monitor_specs(&text, specs, product.section != Section::AllInOnePC);
        }

        if let Some(cpu) = cpu {
            specs["cpu"] = Value::String(cpu);
        }

        if let Some(gpu) = gpu {
            specs["gpu_chipset"] = Value::String(gpu.clone());

            if let Some(gpu_memory) = gpu_memory {
                let memory = format!("{gpu_memory}GB");
                specs["gpu"] = Value::String(format!("{gpu} {memory}"));
                specs["gpu_memory"] = Value::String(memory);
            } else {
                specs["gpu"] = Value::String(gpu);
            }
        }

        if let Some(memory) = memory {
            let memory_size = format!("{memory}GB");
            specs["memory_size"] = Value::String(memory_size.clone());

            let mut found_type = false;
            for memory_type in vec!["DDR3", "DDR4", "DDR5"] {
                if text.contains(memory_type) {
                    specs["memory_type"] = Value::String(memory_type.to_string());
                    specs["memory"] = Value::String(format!("{memory_size} {memory_type}"));
                    found_type = true;
                    break;
                }
            }

            if !found_type {
                specs["memory"] = Value::String(memory_size);
            }
        }

        if let Some(storage) = storage {
            let storage_size = match storage >= 1000 {
                true => format!("{:.1}TB", storage as f64 / 1024.0).replace(".0T", "T"),
                false => format!("{storage}GB")
            };
            specs["storage_size"] = Value::String(storage_size.clone());

            if text.contains("NVME") {
                specs["storage"] = Value::String(format!("{storage_size} NVME"));
            } else if text.contains("SSD") {
                specs["storage"] = Value::String(format!("{storage_size} SSD"));
            } else if text.contains("HDD") {
                specs["storage"] = Value::String(format!("{storage_size} HDD"));
            } else {
                specs["storage"] = Value::String(storage_size);
            }
        }
    }

    fn post_processing(&self, product: &mut Product) {
        if product.section == Section::GamingLaptop {
            if product.name != product.title {
                product.specs["model"] = Value::String(product.name.clone());
            }
        }
    }
}

fn extract_cpu(text: &str) -> (Option<String>, Option<DatasetEntry>) {
    let section = Section::CPU;
    let text = section.config().title_cleaner.replace_words(&text, true);

    // Search in Dataset
    for entry in section.parser().dataset() {
        if text.contains(&entry.name.to_uppercase()) {
            return (Some(entry.name.clone()), Some(entry.clone()));
        }
    }

    // Search in Dataset (Cleaning Text)
    let words_to_remove = &["AMD ", "INTEL ", "CORE ", "GOLD ", "SILVER ", "PROCESSOR ", " GEMINI LAKE", " JASPER LAKE"];
    let cleaned_text = remove_words(&text, words_to_remove);
    for entry in section.parser().dataset() {
        let entry_name = remove_words(&entry.name.to_uppercase(), words_to_remove);
        if cleaned_text.contains(&entry_name) {
            return (Some(entry.name.clone()), Some(entry.clone()));
        }
    }

    // Search for Intel CPUS
    for cpu in vec!["i3", "i5", "i7", "i9", "Ultra 5", "Ultra 7", "Ultra 9", "Core 3", "Core 5", "Core 7", "Core 9"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            let mut cpu = match cpu.contains("Core") {
                true => format!("Intel {cpu}"),
                false => format!("Intel Core {cpu}")
            };

            if cpu.contains("i") {
                let generation_pattern = r"(?i)\b([2-9]|1[0-4])\s*(?:th|é|e|è|éme|ème|eme|gén|gen)\s*(?:gen|gén|generation|génération)?\b";
                if let Some(caps) = RegexCache::captures(&generation_pattern, &text) {
                    if let Some(generation) = caps.get(1).and_then(|v| v.as_str().parse::<i32>().ok()) {
                        cpu.push_str(&format!(" {generation}th Generation"));
                    }
                }
            }

            return (Some(cpu), None);
        }
    }

    // Search for AMD CPUS
    for cpu in vec!["Ryzen 3", "Ryzen 5", "Ryzen 7", "Ryzen 9", "Ryzen"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            return (Some(format!("AMD {cpu}")), None);
        }
    }

    // Search for Other CPUS
    for cpu in vec!["Quad Core", "Quad-Core", "Dual Core", "Dual-Core", "Arm"] {
        if RegexCache::matches(&format!("(?i){cpu}"), &text) {
            let cpu = cpu.replace("-", " ");
            return if text.contains("INTEL") {
                (Some(format!("Intel {cpu} Processor")), None)
            } else if text.contains("AMD") {
                (Some(format!("AMD {cpu} Processor")), None)
            } else {
                (Some(format!("{cpu} Processor")), None)
            };
        }
    }


    if text.contains("INTEL ") {
        (Some("Intel Processor".to_string()), None)
    } else if text.contains("AMD ") {
        (Some("AMD Processor".to_string()), None)
    } else {
        (None, None)
    }
}

fn extract_gpu(text: &str, cpu_entry: &Option<DatasetEntry>) -> (Option<String>, Option<ChipsetEntry>) {
    let section = Section::GPU;
    let text = section.config().title_cleaner.replace_words(&text, true);
    let mut has_dedicated_gpu = vec!["RTX", "RX", "ARC", "GTX", " GT"].into_iter().any(|w| text.contains(w));

    // Search in Dataset
    for chipset in section.parser().chipsets() {
        // Skip IGPU if product has Dedicated GPU
        if has_dedicated_gpu && chipset.name.contains("Graphics") {
            continue;
        }

        if text.contains(&chipset.name.to_uppercase()) {
            return (Some(chipset.name.clone()), Some(chipset.clone()));
        }
    }

    // Search in Dataset (Cleaning Text)
    let words_to_remove = &["GEFORCE ", "RADEON ", "NVIDIA ", "INTEL ", " GRAPHICS"];
    let cleaned_text = remove_words(&text, words_to_remove);
    for chipset in section.parser().chipsets() {
        // Skip IGPU if product has Dedicated GPU
        if has_dedicated_gpu && chipset.name.contains("Graphics") {
            continue;
        }

        let chipset_name = remove_words(&chipset.name.to_uppercase(), words_to_remove);
        if cleaned_text.contains(&chipset_name) {
            return (Some(chipset.name.clone()), Some(chipset.clone()));
        }
    }

    // Extract IGPU from CPU Entry
    if let Some(entry) = &cpu_entry {
        if let Some(iGPU) = entry.data.get("integrated_gpu").and_then(|s| s.as_str()) {
            if !iGPU.is_empty() {
                return (Some(iGPU.to_string()), None);
            }
        }
    }

    if text.contains("INTEL ") {
        if text.contains(" ARC ") {
            (Some("Intel Arc Graphics".to_string()), None)
        } else if text.contains("UHD GRAPHICS") {
            (Some("Intel UHD Graphics".to_string()), None)
        } else if text.contains("HD GRAPHICS") {
            (Some("Intel HD Graphics".to_string()), None)
        } else {
            (Some("Intel Graphics".to_string()), None)
        }
    } else if text.contains("AMD ") || text.contains("RYZEN ") {
        (Some("AMD Radeon Graphics".to_string()), None)
    } else {
        (None, None)
    }
}

fn extract_monitor_specs(text: &str, specs: &mut Value, laptop: bool) {
    let mut specs_list = Vec::new();

    let size_pattern = "(?i)\\b(\\d+(?:[.,]\\d+)?)\\s*(?:″|\"|inch|pouce)";
    for caps in RegexCache::captures_iter(size_pattern, text) {
        if let Some(size_str) = caps.get(1).map(|v| v.as_str()) {
            if let Ok(size) = size_str.replace(",", ".").parse::<f32>() {
                // filter impossible screen sizes
                let range = if laptop { 10.0..=18.0 } else { 10.0..=120.0 };
                if !range.contains(&size) {
                    continue;
                }

                let size = format!("{}\"", size_str.replace(".0", ""));
                specs["display_size"] = Value::String(size.clone());
                specs_list.push(size);
                break;
            }
        }
    }

    let resolution_pattern = r"(?i)\b(FULL\s*HD|FHD|QHD|UHD|HD|2K|2[.,]5K|2[.,]8K|3K|4K|5K|WUXGA|WQXGA|WXGA|WQHD)\b";
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

            specs["resolution"] = Value::String(resolution.clone());
            specs_list.push(resolution);
            break;
        }
    };

    let panel_pattern = r"(?i)\b(IPS|OLED)\b";
    if let Some(caps) = RegexCache::captures(panel_pattern, text) {
        if let Some(panel) = caps.get(1).and_then(|v| Some(v.as_str().to_string())) {
            specs["panel_type"] = Value::String(panel.clone());
            specs_list.push(panel);
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
        specs["refresh_rate"] = Value::String(refresh_rate.clone());
        specs_list.push(refresh_rate);
    }

    if !specs_list.is_empty() {
        specs["display"] = Value::String(specs_list.join(" "));
    }
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