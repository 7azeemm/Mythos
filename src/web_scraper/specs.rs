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
            Self::PC(pc) => vec![
                ("cpu".to_string(), pc.cpu.name.clone())
            ],
        }
    }
}