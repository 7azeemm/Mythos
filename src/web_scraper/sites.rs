use crate::web_scraper::parsers::pc_parser::parse_specs;
use std::error::Error;
use crate::web_scraper::specs::ProductSpecs;

pub mod tunisianet;

pub static PARSERS: &[(Section, fn(&str) -> Result<ProductSpecs, Box<dyn Error>>)] = &[
    (Section::PC, parse_specs),
    (Section::GamingPc, parse_specs),
    (Section::PcAllInOne, parse_specs),
    (Section::GamingSetup, parse_specs),
    (Section::Laptop, parse_specs),
    (Section::GamingLaptop, parse_specs),
    (Section::ProLaptop, parse_specs),
];

pub enum Section {
    PC,
    GamingPc,
    PcAllInOne,
    GamingSetup,
    Laptop,
    GamingLaptop,
    ProLaptop,
}

impl Section {
    pub fn to_str(&self) -> String {
        match self {
            Section::PC => "pc".to_string(),
            Section::GamingPc => "gaming_pc".to_string(),
            Section::PcAllInOne => "pc_all_in_one".to_string(),
            Section::GamingSetup => "gaming_setup".to_string(),
            Section::Laptop => "laptop".to_string(),
            Section::GamingLaptop => "gaming_laptop".to_string(),
            Section::ProLaptop => "pro_laptop".to_string(),
        }
    }
}