use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::web_scraper::specs::component_specs::ComponentSpecs;
use crate::web_scraper::specs::pc_specs::{CpuInfo, PCSpecs};

pub mod pc_specs;
pub mod component_specs;
pub mod cache;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductSpecs {
    PC(PCSpecs),
    Component(ComponentSpecs)
}

impl ProductSpecs {
    pub fn get_filters(&self) -> Vec<(String, String)> {
        match self {
            Self::PC(pc) => {
                let mut filters = vec![];
                if let Some(cpu) = &pc.cpu {
                    filters.push(("cpu".to_string(), match cpu {
                        CpuInfo::Parsed(specs) => specs.name.clone(),
                        CpuInfo::Raw(name) => name.clone()
                    }))
                }
                if let Some(gpu) = &pc.gpu {
                    filters.push(("gpu".to_string(), gpu.name.clone()))
                }
                if let Some(memory) = &pc.memory {
                    filters.push(("ram".to_string(), format!("{} GB", memory.size)));
                    if let Some(ram_type) = &memory.ram_type {
                        filters.push(("ram_type".to_string(), ram_type.clone()));
                    }
                }
                if let Some(storage) = &pc.storage {
                    filters.push(("storage".to_string(), format!(
                        "{} {} {} {}", storage.size, storage.size_unit, storage.storage_type, storage.interface
                    )))
                }
                if let Some(os) = &pc.os {
                    filters.push(("os".to_string(), os.clone()));
                }

                filters
            },
            Self::Component(component) => {
                vec![]
            }
        }
    }
}