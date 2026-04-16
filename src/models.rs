pub struct PC {
    pub cpu: CPU,
    pub motherboard: Motherboard,
    pub memory: Memory,
    pub storage: Storage,
    pub graphics_card: GraphicsCard,
    pub cooler: Option<String>,
    pub case: String,
    pub power_supply: String,
    pub has_win11: bool,
}

pub struct CPU {
    pub name: String,
    pub manufacturer: CPUManufacturer,
    pub base_clock: f32,
    pub boost_clock: f32,
    pub core_count: u32,
    pub thread_count: u32,
    pub l1_cache: u32,
    pub l2_cache: u32,
    pub l3_cache: u32,
    pub tdp: u32,
    pub socket: String,
    pub integrated_gpu: Option<String>,
    pub memory_support: Vec<RamType>,
}

pub enum CPUManufacturer {
    Intel,
    AMD
}

pub struct Motherboard {

}

pub struct Memory {
    pub total_gb: u32,
    pub sticks: u32,
    pub per_stick_gb: u32,
    pub ram_type: RamType,
}

pub enum RamType {
    DDR3,
    DDR4,
    DDR5
}

impl RamType {
    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_uppercase().as_str() {
            "DDR3" => Some(RamType::DDR3),
            "DDR4" => Some(RamType::DDR4),
            "DDR5" => Some(RamType::DDR5),
            _ => None,
        }
    }
}

pub struct Storage {

}

pub struct GraphicsCard {

}