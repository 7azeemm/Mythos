use crate::Error;
use crate::web_scraper::parsers::component_parser::parse_component;
use crate::web_scraper::parsers::pc_parser::parse_pc;
use crate::web_scraper::specs::ProductSpecs;

macro_rules! define_sections {
    (
        $( $variant:ident => ($name:expr, $parser:expr, $requires_desc:expr) ),* $(,)?
    ) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum Section {
            $( $variant ),*
        }

        impl Section {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $( Section::$variant => $name ),*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $( $name => Some(Section::$variant), )*
                    _ => None,
                }
            }

            pub fn parser(&self) -> fn(&str) -> Result<ProductSpecs, Box<dyn Error>> {
                match self {
                    $( Section::$variant => $parser ),*
                }
            }

            pub fn requires_description(&self) -> bool {
                match self {
                    $( Section::$variant => $requires_desc ),*
                }
            }
        }
    };
}

//TODO: add other things like Fan
//TODO: combine ssd and hdd into Storage
define_sections! {
    PC          => ("pc", parse_pc, true),
    GamingPc    => ("gaming_pc", parse_pc, true),
    PcAllInOne  => ("pc_all_in_one", parse_pc, true),
    GamingSetup => ("gaming_setup", parse_pc, true),

    Laptop       => ("laptop", parse_pc, true),
    GamingLaptop => ("gaming_laptop", parse_pc, true),
    ProLaptop    => ("pro_laptop", parse_pc, true),

    Monitor      => ("monitor", parse_component, true),
    Mouse        => ("mouse", parse_component, false),
    KeyBoard     => ("keyboard", parse_component, false),

    CPU         => ("cpu", parse_component, false),
    GPU         => ("gpu", parse_component, false),
    RAM         => ("ram", parse_component, false),
    MotherBoard => ("motherboard", parse_component, false),
    HDD         => ("hdd", parse_component, false),
    SSD         => ("ssd", parse_component, false),
    Cooler      => ("cooler", parse_component, false),
    Case        => ("case", parse_component, false),
    PSU         => ("psu", parse_component, false),
}