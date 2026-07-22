use crate::utils::file_loader::FileLoader;
use crate::utils::regex_cache::RegexCache;
use crate::web_scraper::parsers::gpu_parser::GPUParser;
use crate::web_scraper::parsers::memory_parser::MemoryParser;
use crate::web_scraper::parsers::monitor_parser::MonitorParser;
use crate::web_scraper::parsers::pc_parser::PCParser;
use crate::web_scraper::parsers::storage_parser::StorageParser;
use crate::web_scraper::parsers::{GenericSectionParser, SectionParser};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::{Display, EnumIter, EnumString};
use crate::web_scraper::dataset::Dataset;
use crate::web_scraper::parsers::console_game_parser::ConsoleGameParser;
use crate::web_scraper::parsers::console_parser::ConsoleParser;
use crate::web_scraper::parsers::headphones_parser::HeadphonesParser;
use crate::web_scraper::parsers::keyboard_parser::KeyboardParser;
use crate::web_scraper::parsers::mouse_parser::MouseParser;
use crate::web_scraper::parsers::power_supply_parser::PowerSupplyParser;
use crate::web_scraper::parsers::television_parser::TelevisionParser;

pub static SECTION_PARSERS: OnceCell<HashMap<Section, Arc<dyn SectionParser>>> = OnceCell::new();

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, EnumIter)]
pub enum Section {
    PC,
    GamingPC,
    AllInOnePC,
    MiniPC,

    Laptop,
    GamingLaptop,
    MacBook,

    Monitor,

    CPU,
    GPU,
    Memory,
    Storage,
    Motherboard,
    Cooler,
    PowerSupply,
    Case,

    Mouse,
    Keyboard,
    MousePad,
    Headphones,
    GamingChair,
    AccessoriesCombo,
    UpgradeKit,

    Console,
    Controller,
    ConsoleGame,
    ConsoleAccessories,

    Smartphone,
    Tablet,
    Smartwatch,
    Television,

    Others
}

impl Default for Section {
    fn default() -> Self {
        Self::Others
    }
}

impl Section {
    pub fn is_laptop(&self) -> bool {
        matches!(self, Self::Laptop | Self::GamingLaptop | Self::MacBook)
    }

    pub fn is_low_priority(&self) -> bool {
        matches!(self, Self::PC | Self::Laptop | Self::AccessoriesCombo)
    }
    
    pub fn requires_desc(&self) -> bool {
        matches!(self,
            Self::PC | Self::GamingPC | Self::AllInOnePC | Self::MiniPC | Self::Laptop |
            Self::GamingLaptop | Self::MacBook | Self::MacBook
        )
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
    pub id: Section,
    pub force_include: Vec<String>,
    pub move_rules: Vec<Vec<String>>,
    pub move_by_description_rules: Vec<Vec<String>>,
    pub min_price: i32,
    pub title_cleaner: TitleCleanerConfig,
    pub brands: Vec<String>,
    pub filters: Vec<String>,
    pub components: Vec<String>,
    pub group: Vec<String>,
    pub datasets: HashMap<String, Section>,
    #[serde(default = "default_id_field_name")]
    pub id_field_name: String
}

fn default_id_field_name() -> String {
    "name".to_string()
}

impl SectionConfig {
    pub async fn load() {
        let mut parsers: HashMap<Section, Arc<dyn SectionParser>> = HashMap::new();

        for config in FileLoader::load_or_default::<Vec<SectionConfig>>("config/sections.json").await.unwrap() {
            let section = config.id;
            let config = Arc::new(config);
            let dataset = Dataset::load(section).await.unwrap();

            let parser: Arc<dyn SectionParser> = match section {
                Section::GPU => Arc::new(GPUParser { config, dataset }),
                Section::Memory => Arc::new(MemoryParser { config, dataset }),
                Section::Storage => Arc::new(StorageParser { config, dataset }),
                Section::PowerSupply => Arc::new(PowerSupplyParser { config, dataset }),
                Section::Mouse => Arc::new(MouseParser { config, dataset }),
                Section::Keyboard => Arc::new(KeyboardParser { config, dataset }),
                Section::Headphones => Arc::new(HeadphonesParser { config, dataset }),
                Section::Console => Arc::new(ConsoleParser { config, dataset }),
                Section::ConsoleGame => Arc::new(ConsoleGameParser { config, dataset }),
                Section::Monitor => Arc::new(MonitorParser { config, dataset }),
                Section::Television => Arc::new(TelevisionParser { config, dataset }),
                Section::PC | Section::GamingPC | Section::AllInOnePC | Section::MiniPC |
                Section::Laptop | Section::GamingLaptop | Section::MacBook => Arc::new(PCParser { config, dataset }),
                _ => Arc::new(GenericSectionParser { config, dataset })
            };

            parsers.insert(section, parser);
        }

        SECTION_PARSERS.set(parsers).ok();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TitleCleanerConfig {
    pub remove_words: Vec<String>,
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