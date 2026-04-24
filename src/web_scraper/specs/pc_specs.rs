use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCSpecs {
    pub cpu: CPUSpecs,
    pub motherboard: String,
    pub memory: MemorySpecs,
    pub storage: StorageSpecs,
    pub gpu: GPUSpecs,
    pub cooler: Option<String>,
    pub case: String,
    pub psu: String,
    pub monitor: Option<String>,
    pub os: Option<String>,
    pub warranty: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CPUSpecs {
    pub name: String,
    pub base_clock: f32,
    pub boost_clock: f32,
    pub core_count: u32,
    pub thread_count: u32,
    pub l1_cache: f32,
    pub l2_cache: f32,
    pub l3_cache: f32,
    pub tdp: u32,
    pub socket: String,
    #[serde(deserialize_with = "deserialize_option")]
    pub integrated_gpu: Option<String>,
    #[serde(deserialize_with = "deserialize_memory_support")]
    pub memory_support: Vec<RamType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GPUSpecs {
    pub name: String,
    pub base_clock: u32,
    pub boost_clock: u32,
    #[serde(deserialize_with = "deserialize_option")]
    pub memory_size: Option<u32>,
    #[serde(deserialize_with = "deserialize_option")]
    pub memory_type: Option<GraphicsMemoryType>,
    #[serde(deserialize_with = "deserialize_option")]
    pub memory_bus: Option<u32>,
    #[serde(deserialize_with = "deserialize_option")]
    pub memory_bandwidth: Option<u32>,
    #[serde(deserialize_with = "deserialize_option")]
    pub bus_interface: Option<String>,
    pub transistors: f32,
    pub cores: u32,
    pub tensor_cores: u32,
    pub rt_cores: u32,
    pub t_flops: f32,
    pub tdp: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GraphicsMemoryType {
    GDDR6,
    GDDR7
}

impl FromStr for GraphicsMemoryType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "GDDR6" => Ok(GraphicsMemoryType::GDDR6),
            "GDDR7" => Ok(GraphicsMemoryType::GDDR7),
            _ => Err(format!("Unknown graphics memory type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemorySpecs {
    pub size: u32,
    pub sticks: u32,
    pub ram_type: RamType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RamType {
    DDR3,
    DDR4,
    DDR5
}

impl FromStr for RamType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "DDR3" => Ok(RamType::DDR3),
            "DDR4" => Ok(RamType::DDR4),
            "DDR5" => Ok(RamType::DDR5),
            _ => Err(format!("Unknown memory type: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageSpecs {
    pub storage_type: StorageType,
    pub size: u32,
    pub interface: StorageInterface,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageType {
    HDD,
    SSD,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StorageInterface {
    SATA,
    NVMe
}

fn deserialize_memory_support<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<RamType>, D::Error> {
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',')
        .map(|part| RamType::from_str(part).expect(&format!("Unknown RAM type: {s}")))
        .collect())
}

fn deserialize_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() || s.trim().to_uppercase() == "N/A" {
        Ok(None)
    } else {
        T::from_str(s.trim())
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}