use crate::web_scraper::parsers::pc_parser::parse_pc;
use std::error::Error;
use crate::web_scraper::specs::ProductSpecs;

pub mod tunisianet;

pub static PARSERS: &[(Section, fn(&str) -> Result<ProductSpecs, Box<dyn Error>>)] = &[
    (Section::PC, parse_pc),
    (Section::GamingPc, parse_pc),
    (Section::PcAllInOne, parse_pc),
    (Section::GamingSetup, parse_pc),
    (Section::Laptop, parse_pc),
    (Section::GamingLaptop, parse_pc),
    (Section::ProLaptop, parse_pc),
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