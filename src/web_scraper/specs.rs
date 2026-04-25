use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::web_scraper::specs::pc_specs::PCSpecs;

pub mod pc_specs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductSpecs {
    PC(PCSpecs),
}

impl ProductSpecs {
    pub fn get_filters(&self) -> Vec<(String, String)> {
        match self {
            Self::PC(pc) => {
                let mut filters = vec![
                    ("cpu".to_string(), pc.cpu.name.clone()),
                    ("gpu".to_string(), pc.gpu.name.clone()),
                    ("ram_type".to_string(), format!("{:?}", pc.memory.ram_type)),
                    ("ram_size".to_string(), format!("{}", pc.memory.size)),
                    ("storage_type".to_string(), format!("{:?}", pc.storage.storage_type)),
                ];

                if let Some(os) = &pc.os {
                    filters.push(("os".to_string(), os.clone()));
                }

                filters
            }
        }
    }
}