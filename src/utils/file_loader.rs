use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use serde_json::{json, Value};
use tokio::fs::{create_dir_all, read_to_string, write};
use crate::utils::serde_ext::JsonExt;

pub struct FileLoader;

impl FileLoader {
    pub async fn load_from_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
        let content = read_to_string(path)
            .await
            .map_err(|e| format!("Failed to load {path}: {e}"))?;
        Ok(serde_json::from_str(&content).map_err(|e| format!("Failed to load {path}: {e}"))?)
    }

    pub async fn load_or_default<T: DeserializeOwned + Serialize + Default>(
        path: &str,
    ) -> Result<T, String> {
        match read_to_string(path).await {
            Ok(content) => {
                // File exists, parse the content
                serde_json::from_str(&content).map_err(|e| format!("Failed to parse {path}: {e}"))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Self::create_path_if_absent(path).await?;

                // File not found, create default content
                let default_val = T::default();
                let json = serde_json::to_string_pretty(&default_val)
                    .map_err(|e| format!("Failed to serialize default content for {path}: {e}"))?;

                write(path, json)
                    .await
                    .map_err(|e| format!("Failed to create default file at {path}: {e}"))?;

                Ok(default_val)
            }
            Err(e) => Err(format!("Failed to load {path}: {e}")),
        }
    }

    pub async fn save_to_file<T: Serialize>(path: &str, data: &T) -> Result<(), String> {
        Self::create_path_if_absent(path).await?;

        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize data for {path}: {e}"))?;

        write(path, json)
            .await
            .map_err(|e| format!("Failed to write to {path}: {e}"))?;

        Ok(())
    }

    async fn create_path_if_absent(path: &str) -> Result<(), String> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                create_dir_all(parent)
                    .await
                    .map_err(|e| format!("Failed to create path for {path}: {e}"))?;
            }
        }
        Ok(())
    }

    pub async fn load_csv(path: &str) -> Result<Vec<Value>, String> {
        let file = File::open(path).map_err(|err| format!("Failed to open file: {err}"))?;
        let mut csv_reader = csv::Reader::from_reader(file);

        let headers = csv_reader.headers()
            .map_err(|err| format!("Failed to read headers: {err}"))?
            .clone();

        let mut records = Vec::new();
        for result in csv_reader.records() {
            let record = result.map_err(|err| format!("Failed to read record: {err}"))?;

            let mut obj = serde_json::Map::new();
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), Value::String(field.to_string()));
                }
            }

            records.push(Value::Object(obj));
        }

        if path == "config/datasets/GPU.csv" {
            let brands = &mut [
                ("Arc", vec!["Arc B570", "Arc B580", "130V", "140V", "130T", "140T", "A370M"]),
                ("GT ", vec!["GT 610", "GT 630", "GT 710", "GT 730", "GT 1030"]),
                ("GTX", vec!["GTX 750", "GTX 950", "GTX 1050", "GTX 1060", "GTX 1070", "GTX 1080", "GTX 1650", "GTX 1660", "GTX 1660 SUPER", "GTX 1660 Ti"]),
                ("RTX", vec!["A1000", "A2000", "A3000", "2000", "A400", "500", "RTX 2050", "RTX 2060", "RTX 2060 SUPER", "RTX 2070", "RTX 2070 SUPER", "RTX 2080", "RTX 2080 SUPER", "RTX 2080 Ti", "RTX 3050", "RTX 3050 Ti", "RTX 3060", "RTX 3060 Ti", "RTX 3070", "RTX 3070 Ti", "RTX 3080", "RTX 3080 Ti", "RTX 3090", "RTX 3090 Ti", "RTX 4050", "RTX 4060", "RTX 4060 Ti", "RTX 4070", "RTX 4070 SUPER", "RTX 4070 Ti", "RTX 4070 Ti SUPER", "RTX 4080", "RTX 4080 SUPER", "RTX 4090", "RTX 5050", "RTX 5060", "RTX 5060 Ti", "RTX 5070", "RTX 5070 SUPER", "RTX 5070 Ti", "RTX 5070 Ti SUPER", "RTX 5080", "RTX 5080 SUPER", "RTX 5090"]),
                ("Radeon", vec!["Vega 6", "Vega 7", "Vega 8", "Vega 11", "RX 580", "RX 6500 XT", "RX 6600 XT", "RX 6650 XT", "RX 6700", "RX 6700 XT", "RX 6750 XT", "RX 6800", "RX 6800 XT", "RX 6850M XT", "RX 6900 XT", "RX 6950 XT", "RX 7600", "RX 7600 XT", "RX 7700", "RX 7700 XT", "RX 7800 XT", "RX 7900 XT", "RX 7900 XTX", "RX 9060", "RX 9060 XT", "RX 9060 XT LP", "RX 9070", "RX 9070 XT", "530", "610M", "660M", "680M", "740M", "760M", "840M", "860M", "880M", "Vega", "FirePro D300"]),
                ("Qualcomm Adreno", vec![]),
                ("Iris", vec!["Plus Graphics 645", "Plus Graphics 655", "Plus", "Pro", "Xe"]),
                ("Intel HD Graphics", vec!["510", "530", "620", "630", "2000", "3000", "4000", "4400", "4600", "5000"]),
                ("Intel UHD Graphics", vec!["610", "630", "710", "730", "750", "770"]),
                ("Nvidia", vec!["MX110", "MX130", "MX150", "MX230", "MX330", "MX350", "MX450", "MX550", "MX570A", "920M", "920MX", "940MX", "Quadro", "Quadro P400", "Quadro P600", "Quadro P620", "Quadro P1000", "Quadro P2000", "Quadro P2200", "Quadro P4000", "Quadro P5000", "Quadro P6000"]),
            ];

            let mut map: HashMap<String, Value> = HashMap::new();

            for record in &records {
                let name = record.get_str("name").unwrap();

                let mut found_brand = false;
                for (brand, variants) in brands.iter_mut() {
                    if !name.contains(*brand) {
                        continue;
                    }
                    found_brand = true;

                    if variants.is_empty() {
                        map.entry((*brand).to_string())
                            .or_insert_with(|| Value::Array(Vec::new()))
                            .as_array_mut()
                            .unwrap()
                            .push(Value::String(name.to_string()));
                        continue
                    } else {
                        let object = map
                            .entry((*brand).to_string())
                            .or_insert_with(|| Value::Object(serde_json::Map::new()))
                            .as_object_mut()
                            .unwrap();

                        let mut found = false;
                        variants.sort_by_key(|s| Reverse(s.len()));
                        for variant in variants {
                            if name.contains(*variant) {
                                found = true;
                                object.entry((*variant).to_string())
                                    .or_insert_with(|| Value::Array(Vec::new()))
                                    .as_array_mut()
                                    .unwrap()
                                    .push(Value::String(name.to_string()));
                                break;
                            }
                        }

                        if found {
                            continue
                        } else {
                            object.entry("Others".to_string())
                                .or_insert_with(|| Value::Array(Vec::new()))
                                .as_array_mut()
                                .unwrap()
                                .push(Value::String(name.to_string()));
                        }
                    }

                    break;
                }

                if !found_brand && !name.is_empty() {
                    map.entry("Others".to_string())
                        .or_insert_with(|| Value::Array(Vec::new()))
                        .as_array_mut()
                        .unwrap()
                        .push(Value::String(name.to_string()));
                }
            }

            Self::save_to_file("GPU.json", &map).await?;
        }

        Ok(records)
    }
}
