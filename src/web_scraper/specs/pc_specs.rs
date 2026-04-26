use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCSpecs {
    pub cpu: Option<CpuInfo>,
    pub gpu: Option<GpuInfo>,
    pub motherboard: Option<String>,
    pub memory: Option<MemoryInfo>,
    pub storage: Option<StorageInfo>,
    pub cooler: Option<String>,
    pub case: Option<String>,
    pub psu: Option<String>,
    pub monitor: Option<String>,
    pub os: Option<String>,
    pub warranty: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CpuInfo {
    Parsed(CPUSpecs),
    Raw(String),
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
    // #[serde(deserialize_with = "deserialize_memory_support")]
    // pub memory_support: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub full_name: String,
    pub memory: Option<String>,
    pub memory_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryInfo {
    pub size: u32,
    pub sticks: Option<u32>,
    pub ram_type: Option<String>,
    pub speed: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageInfo {
    pub size: u32,
    pub size_unit: String,
    pub storage_type: String,
    pub interface: String,
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