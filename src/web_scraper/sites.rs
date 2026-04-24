use crate::web_scraper::parsers::pc_parser::parse_specs;
use std::error::Error;
use crate::web_scraper::specs::ProductSpecs;

pub mod tunisianet;

pub static PARSERS: &[(Section, fn(&str) -> Result<ProductSpecs, Box<dyn Error>>)] = &[
    (Section::PC, parse_specs),
    (Section::GamingPc, parse_specs)
];

pub enum Section {
    PC,
    GamingPc,
}

impl Section {
    pub fn to_str(&self) -> String {
        match self {
            Section::PC => "pc".to_string(),
            Section::GamingPc => "gaming_pc".to_string()
        }
    }
}