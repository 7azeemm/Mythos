use crate::utils::file_loader::FileLoader;
use crate::web_scraper::parsers::gpu_parser::GPUParser;
use crate::web_scraper::parsers::memory_parser::MemoryParser;
use crate::web_scraper::parsers::storage_parser::StorageParser;
use crate::web_scraper::parsers::{GenericSectionParser, SectionParser};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Postgres};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::{Display, EnumIter, EnumString};
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::parsers::pc_parser::PCParser;

pub static SECTION_PARSERS: OnceCell<HashMap<Section, Arc<dyn SectionParser>>> = OnceCell::new();

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Section {
    PC,
    GamingPC,
    AllInOnePC,
    MiniPC,

    Laptop,
    GamingLaptop,
    MacBook,

    Monitor,
    Mouse,
    Keyboard,
    AccessoriesCombo,
    UpgradeKit,

    CPU,
    GPU,
    Memory,
    Motherboard,
    Storage,
    Case,
    PowerSupply,
    Cooler,

    Trash
}

impl Section {
    pub fn list() -> Vec<Section> {
        vec![
            Self::Case, Self::PowerSupply,
            Self::MiniPC, Self::AllInOnePC, Self::UpgradeKit,
            Self::MacBook, Self::GamingLaptop, Self::Laptop,
            Self::GamingPC, Self::PC, Self::Monitor,
            Self::AccessoriesCombo, Self::Mouse, Self::Keyboard,
            Self::CPU, Self::GPU, Self::Memory, Self::Motherboard, Self::Storage, Self::Cooler
        ]
    }

    pub fn is_low_priority(&self) -> bool {
        matches!(self, Self::PC | Self::Laptop | Self::AccessoriesCombo)
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|err| format!("Section `{s}` not found"))
    }

    pub fn parser(&self) -> Arc<dyn SectionParser> {
        SECTION_PARSERS.get().unwrap().get(&self).cloned()
            .expect(&format!("Section `{self}` not found in the config file"))
    }

    pub fn config(&self) -> Arc<SectionConfig> {
        self.parser().config()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SectionConfig {
    pub id: String,
    pub force_include: Vec<String>,
    pub include: Vec<String>,
    pub include_description: Vec<String>,
    pub exclude: Vec<String>,
    pub exclude_description: Vec<String>,
    pub switchable_to: Vec<String>,
    pub unswitchable: bool,
    pub skip_include_check: bool,
    pub has_chipsets_dataset: bool,
    pub requires_description: bool,
    pub optional_dataset_words: Vec<String>,
    pub title_cleaner: TitleCleanerConfig,
    pub brands: Vec<String>,
    pub filters: Vec<String>,
    pub render_specs: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TitleCleanerConfig {
    pub remove_words: Vec<String>,
    pub remove_patterns: Vec<String>,
    pub replace_words: Vec<Vec<String>>,
    pub add_brands_by_models: Vec<Vec<String>>
}

impl TitleCleanerConfig {
    pub fn replace_words(&self, text: &str, uppercase: bool) -> String {
        let mut text = text.to_string();
        for list in &self.replace_words {
            if let (Some(from), Some(to)) = (list.first(), list.get(1)) {
                let string = RegexCache::replace_all(&format!("(?i){}", regex::escape(from)), &text, to);
                text = if uppercase { string.to_uppercase() } else { string.to_string() }
            }
        }
        text
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetEntry {
    pub name: String,
    pub data: Value,
    pub chipset: Option<ChipsetEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChipsetEntry {
    pub name: String,
    pub data: Value,
}

impl SectionConfig {
    pub async fn load() -> Result<(), Box<dyn Error>> {
        let mut parsers: HashMap<Section, Arc<dyn SectionParser>> = HashMap::new();
        let configs = FileLoader::load_or_create::<Vec<SectionConfig>>("config/sections.json").await?;

        for config in configs {
            let section = Section::from_str(&config.id)?;
            let config = Arc::new(config);
            let (dataset, chipsets) = config.load_datasets().await?;

            let parser: Arc<dyn SectionParser> = match section {
                Section::GPU => Arc::new(GPUParser { config, dataset, chipsets }),
                Section::Memory => Arc::new(MemoryParser { config, dataset }),
                Section::Storage => Arc::new(StorageParser { config, dataset }),
                Section::PC | Section::GamingPC | Section::AllInOnePC | Section::MiniPC |
                Section::Laptop | Section::GamingLaptop | Section::MacBook => Arc::new(PCParser { config, dataset }),
                _ => Arc::new(GenericSectionParser { config, dataset })
            };

            parsers.insert(section, parser);
        }

        SECTION_PARSERS.set(parsers).ok();

        Ok(())
    }

    async fn load_datasets(&self) -> Result<(Vec<DatasetEntry>, Vec<ChipsetEntry>), String> {
        let mut dataset_entries = Vec::new();
        let mut chipset_entries = Vec::new();

        let dataset_path = format!("config/datasets/{}.csv", self.id);
        if Path::new(&dataset_path).exists() {
            for entry in FileLoader::load_csv(&dataset_path)? {
                let name = entry.get("name").unwrap().as_str().unwrap().to_string();
                if name.is_empty() {
                    continue
                }
                dataset_entries.push(DatasetEntry {
                    name,
                    data: entry,
                    chipset: None
                });
            }
        }

        // For GPUs only
        if self.has_chipsets_dataset {
            let chipsets_path = format!("config/datasets/{}-chipsets.csv", self.id);
            if Path::new(&chipsets_path).exists() {
                for entry in FileLoader::load_csv(&chipsets_path)? {
                    let chipset_name = entry.get("name").unwrap().as_str().unwrap().to_string();
                    if chipset_name.is_empty() {
                        continue
                    }

                    for dataset_entry in dataset_entries.iter_mut() {
                        if dataset_entry.name.contains(&chipset_name) {
                            match entry.get("memory_size").and_then(|v| v.as_str()) {
                                None => break,
                                Some(size) => if dataset_entry.name.contains(&format!("{size}GB")) {
                                    dataset_entry.chipset = Some(ChipsetEntry {
                                        name: chipset_name.clone(),
                                        data: entry.clone()
                                    });
                                },
                            }
                        }
                    }

                    chipset_entries.push(ChipsetEntry {
                        name: chipset_name,
                        data: entry
                    });
                }
            }

            for entry in &dataset_entries {
                if entry.chipset.is_none() {
                    eprintln!("GPU Chipset not found: `{}`", entry.name);
                }
            }
        }

        dataset_entries.sort_by(|a, b| b.name.cmp(&a.name));
        chipset_entries.sort_by(|a, b| b.name.cmp(&a.name));

        Ok((dataset_entries, chipset_entries))
    }
}

impl sqlx::Type<Postgres> for Section {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::Type<Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, Postgres> for Section {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as sqlx::Encode<'_, Postgres>>::encode_by_ref(&&self.to_string().as_str(), buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Section {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as sqlx::Decode<'r, Postgres>>::decode(value)?;
        Section::from_str(s).map_err(|e| e.into())
    }
}

impl sqlx::postgres::PgHasArrayType for Section {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}