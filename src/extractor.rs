use std::error::Error;
use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;
use crate::dataset::{CPU_DATASET, GPU_DATASET};
use crate::models::{GPUSpecs, MemorySpecs, RamType, StorageSpecs, StorageInterface, StorageType, CPUSpecs, PCSpecs};

static GPU_PATTERNS: &[&str] = &[
    "AMD", "ASUS", "NVIDIA", "GIGABYTE", "MSI",
    "Graphics", "INNO3D", "TWIN", "VENTUS", "WINDFORCE", "Édition",
    "TRIO", "GAMING", "WHITE", "SAPPHIRE PULSE", "XFX Swift",
    "ZOTAC", "EDGE", "Dual", "INSPIRE", "SHADOW", "BULK",
    "X2", "OC", "V2", "V3", "XS", "2X", "3X", ",",
];

static CPU_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\s+\d+\s*(?:e|ème|eme)?\s+géné.*").unwrap());
static GPU_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"(?i){}", GPU_PATTERNS.join("|"))).unwrap());
static GPU_CLEANUP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\s+\d+\s*G[B]?$").unwrap());
static GPU_MEMORY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\s*Go").unwrap());

pub fn extract_specs(description: &str) -> Result<PCSpecs, Box<dyn Error>> {
    let mut parts = description.split(" - ").collect::<Vec<&str>>().into_iter();

    let first = parts.next().ok_or("description is empty")?;
    let (case, monitor) = match first.contains("Écran") {
        true => (parts.next().ok_or("case line not found")?.to_string(), Some(first.to_string())),
        false => (first.to_string(), None)
    };

    let cpu = parse_cpu(parts.next().ok_or("cpu line not found")?)?;
    let memory = parse_memory(parts.next().ok_or("memory line not found")?)?;
    let storage = parse_storage(parts.next().ok_or("storage line not found")?)?;
    let gpu = parse_gpu(parts.next().ok_or("gpu line not found")?)?;
    let motherboard = parse_motherboard(parts.next().ok_or("motherboard line not found")?)?;
    let mut cooler = None;
    let mut psu = None;
    let mut os = None;
    let mut warranty = None;
    let mut extra = None;

    while let Some(next) = parts.next() {
        if (next.starts_with("Refroidisseur") || next.starts_with("Ventilateur")) &&
            let Some(pos) = next.to_lowercase().find("processeur") {
            cooler = Some(next[pos+10..].trim().to_string());
        } else if next.starts_with("Boîte d'alimentation") {
            psu = Some(next[22..].trim().to_string());
        } else if next.starts_with("Garantie") {
            warranty = Some(next.split_whitespace().nth(1).unwrap().parse::<u32>()?)
        } else if let Some(pos) = next.to_lowercase().find("watercooling") {
            cooler = Some(format!("W{}", next[pos+1..].trim()));
        } else if next.starts_with("Windows") {
            os = Some(next.to_string());
        } else if next.starts_with("Avec") {
            extra = Some(next.replace("Avec ", "")
                .replace("Ensemble ", "")
                .replace("Clavier", "Keyboard")
                .replace("Souris", "Mouse")
                .replace("Casque", "Headset")
                .replace("Noir", "Black"));
        }
    }
    
    let psu = psu.ok_or("psu not found")?;
    
    Ok(PCSpecs {
        cpu, gpu, motherboard, memory, storage,
        cooler, case, psu, monitor, os, warranty
    })
}

fn parse_cpu(input: &str) -> Result<CPUSpecs, Box<dyn Error>> {
    let content = input.strip_prefix("Processeur")
        .ok_or_else(|| format!("invalid cpu line: {input}"))?;
    let name = content.split(',').next().ok_or_else(|| format!("invalid cpu line: {input}"))?;
    let name = CPU_NAME_RE.replace(name, "").trim().to_string();
    CPU_DATASET.get(&name).cloned().ok_or_else(|| format!("cpu not found in dataset: {name}").into())
}

fn parse_storage(input: &str) -> Result<StorageSpecs, Box<dyn Error>> {
    if !input.starts_with("Disque") {
        return Err(format!("invalid storage line: {input}").into());
    }
    let storage_type = if input.contains("SSD") { StorageType::SSD } else { StorageType::HDD };
    let interface = if input.contains("NVMe") { StorageInterface::NVMe } else { StorageInterface::SATA };

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let unit = tokens.last().ok_or_else(|| format!("invalid storage line: {input}"))?;
    let mut size = tokens.get(tokens.len() - 2)
        .ok_or_else(|| format!("invalid storage line: {input}"))?
        .parse::<u32>()?;

    if unit.to_uppercase().starts_with("T") {
        size *= 1024;
    }

    Ok(StorageSpecs {
        storage_type,
        size,
        interface
    })
}

fn parse_memory(input: &str) -> Result<MemorySpecs, Box<dyn Error>> {
    if !input.starts_with("Mémoire") && !input.starts_with("RAM") {
        return Err(format!("invalid memory line: {input}").into());
    }

    let size = input.split_whitespace().nth(1)
        .ok_or_else(|| format!("invalid memory line: {input}"))?
        .parse::<u32>()?;
    let sticks_part = input.split('(').nth(1).ok_or_else(|| format!("invalid memory line: {input}"))?;
    let sticks = sticks_part.chars().take_while(|c| c.is_ascii_digit())
        .collect::<String>().parse()?;
    let ram_type = RamType::from_str(
        input.split_whitespace().last().ok_or_else(|| format!("invalid memory line: {input}"))?
    )?;

    Ok(MemorySpecs {
        size,
        sticks,
        ram_type,
    })
}

fn parse_gpu(input: &str) -> Result<GPUSpecs, Box<dyn Error>> {
    let mut gpu = input.strip_prefix("Carte graphique ")
        .ok_or_else(|| format!("invalid gpu line: {input}"))?
        .to_string();

    if gpu.contains("RX ") && gpu.to_lowercase().contains("vega") {
        gpu = gpu.replace("RX ", "");
    }

    if gpu.contains("GEFORCE") {
        gpu = gpu.replace("GEFORCE", "GeForce");
    }

    let memory = GPU_MEMORY_RE.captures(&gpu)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str());

    let base_name = gpu.split(',').next().unwrap_or(&gpu);
    let name = GPU_NAME_RE.replace_all(base_name, "").trim().to_string();
    let name = GPU_CLEANUP_RE.replace(&name, "").trim().to_string();
    let name = match memory {
        Some(m) => format!("{name} {m} GB"),
        None => name
    };

    GPU_DATASET.get(&name).cloned().ok_or_else(|| format!("gpu not found in dataset: {name}").into())
}

fn parse_motherboard(input: &str) -> Result<String, Box<dyn Error>> {
    input.strip_prefix("Carte mère")
        .or_else(|| input.strip_prefix("Carte mére"))
        .or_else(|| input.strip_prefix("Carte Mère"))
        .ok_or_else(|| format!("invalid motherboard line: {input}").into())
        .map(|s| s.trim().to_string())
}