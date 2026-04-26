use std::cell::RefCell;
use crate::utils::dataset::{CPU_DATASET};
use crate::web_scraper::specs::pc_specs::{CpuInfo, GpuInfo, MemoryInfo, PCSpecs, StorageInfo};
use crate::web_scraper::specs::ProductSpecs;
use regex::Regex;
use std::error::Error;

use super::patterns::*;

pub fn parse_pc(description: &str) -> Result<ProductSpecs, Box<dyn Error>> {
    let mut parts = description.split("- ").collect::<Vec<&str>>().into_iter();

    let mut cpu = None;
    let mut gpu = None;
    let mut memory = None;
    let mut storage = None;
    let mut motherboard = None;
    let mut case = None;
    let mut cooler = None;
    let mut psu = None;
    let mut os = None;
    let mut warranty = None;
    let mut monitor = None;
    let extra = RefCell::new(Vec::new());

    let mut functions: Vec<(Vec<&str>, Box<dyn FnMut(&str)>)> = vec![
        (vec!["processeur", "!refroidisseur", "!ventilateur", "!socket"], Box::new(|input| { cpu = parse_cpu(input); })),
        (vec!["graphique"], Box::new(|input| { gpu = parse_gpu(input); })),
        (vec!["mémoire", "ram", "!lecteur"], Box::new(|input| { memory = parse_memory(input); })),
        (vec!["disque", "!ports"], Box::new(|input| { storage = parse_storage(input); })),
        (vec!["carte mère", "carte mére"], Box::new(|input| { motherboard = parse_motherboard(input);})),
        (vec!["boîtier", "!refroidisseur", "!fin"], Box::new(|input| { case = parse_case(input); })),
        (vec!["écran"], Box::new(|input| { monitor = Some(input.to_string()); })),
        (vec!["refroidisseur", "ventilateur", "watercooling"], Box::new(|input| { cooler = parse_cooler(input); })),
        (vec!["alimentation", "!chargeur", "!bouton"], Box::new(|input| { psu = parse_psu(input); })),
        (vec!["windows", "exploitation", "freedos", "!hello"], Box::new(|input| { os = parse_os(input); })),
        (vec!["avec", "!garantie"], Box::new(|input| { extra.borrow_mut().push(input.to_string()) })),
        (vec!["garantie", "!écran"], Box::new(|input| {
            if let Some((warrant, ext)) = parse_warranty(input) {
                warranty = Some(warrant);
                if let Some(ext) = ext {
                    extra.borrow_mut().push(ext);
                }
            }
        })),
    ];

    while let Some(part) = parts.next() {
        let part_lower = part.to_lowercase();

        let func = functions.iter_mut().find_map(|(keys, func)| {
            let mut matches = false;
            let mut excluded = false;

            for key in keys {
                if let Some(exclude_word) = key.strip_prefix('!') {
                    if part_lower.contains(exclude_word) {
                        excluded = true;
                        break;
                    }
                } else if part_lower.contains(*key) {
                    matches = true;
                }
            }

            if matches && !excluded { Some(func) } else { None }
        });

        let Some(func) = func else {
            extra.borrow_mut().push(part.to_string());
            continue
        };

        (*func)(part);
    }

    drop(functions);

    Ok(ProductSpecs::PC(PCSpecs {
        cpu,
        gpu,
        motherboard,
        memory,
        storage,
        cooler,
        case,
        psu,
        monitor,
        os,
        warranty,
    }))
}

fn parse_cpu(input: &str) -> Option<CpuInfo> {
    let result = CPU_CLEANUP_RE.replace_all(input, "").into_owned();

    match result.split(',').next() {
        Some(name) => match CPU_DATASET.get(name).cloned() {
            Some(cpu) => Some(CpuInfo::Parsed(cpu)),
            None => {
                // eprintln!("Cpu not found in the dataset: {name}");
                Some(CpuInfo::Raw(name.to_string()))
            }
        }
        None => {
            eprintln!("Failed to parse cpu: {input}");
            Some(CpuInfo::Raw(result))
        }
    }
}

fn parse_gpu(input: &str) -> Option<GpuInfo> {
    let mut gpu = GPU_PREFIX_RE.replace(input, "").into_owned();

    let memory = GPU_VRAM_RE.captures(&gpu)
        .and_then(|caps| caps.get(1))
        .map(|m| format!("{} GB", m.as_str()));

    let memory_type = GPU_VRAM_TYPE_RE.captures(&gpu)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_uppercase());

    gpu = GPU_PARENTHESES_RE.replace_all(&gpu, "").into_owned();
    gpu = gpu.replace("coeurs", "cores")
        .replace("GEFORCE", "GeForce")
        .replace("Édition", "Edition");

    let mut is_integrated = false;
    if GPU_INTEGREE_RE.is_match(&gpu) {
        is_integrated = true;
        gpu = GPU_INTEGREE_RE.replace_all(&gpu, "").into_owned();
    }

    if gpu.contains("RX ") && gpu.to_lowercase().contains("vega") {
        gpu = gpu.replace("RX ", "");
    }

    let base_name = gpu.split(',').next().unwrap_or(&gpu);
    let mut name = GPU_VRAM_RE.replace(base_name, "").to_string();
    name = name.replace("Carte graphique ", "").trim().to_string();

    if !name.contains(' ') {
        name = format!("{name} Graphics");
    }

    name = match memory.clone() {
        Some(m) => format!("{name} {m}"),
        None if is_integrated => format!("{name} Integrated"),
        None => name
    };

    let mut full_name = name.trim().to_string();
    if let Some(mt) = memory_type.clone() {
        full_name = format!("{full_name} {mt}");
    }

    name = GPU_NAME_RE.replace_all(&name, "").to_string();
    name = GPU_MULTI_SPACE_RE.replace_all(&name, " ").trim().to_string();

    Some(GpuInfo {
        name,
        full_name,
        memory,
        memory_type
    })
}

fn parse_storage(input: &str) -> Option<StorageInfo> {
    let storage_type = if input.contains("SSD") { "SSD" } else { "HDD" };
    let interface = if input.to_uppercase().contains("NVME") { "NVMe" } else { "SATA" };

    let caps = STORAGE_SIZE_RE.captures(input)?;

    let mut size = caps.get(1)?.as_str().parse::<u32>().ok()?;
    let size_unit = caps.get(2)?.as_str().to_string();

    Some(StorageInfo {
        size,
        size_unit,
        storage_type: storage_type.to_string(),
        interface: interface.to_string()
    })
}

fn parse_memory(input: &str) -> Option<MemoryInfo> {
    let extract_u32 = |re: &Regex| -> Option<u32> {
        re.captures(input)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse().ok())
    };

    let size = extract_u32(&RAM_SIZE_RE)?;
    let sticks = extract_u32(&RAM_STICKS_RE);
    let speed = extract_u32(&RAM_SPEED_RE);

    let ram_type = RAM_TYPE_RE.captures(input)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_uppercase());

    Some(MemoryInfo {
        size,
        sticks,
        ram_type,
        speed,
    })
}

fn parse_motherboard(input: &str) -> Option<String> {
    input.strip_prefix("Carte mère")
        .or_else(|| input.strip_prefix("Carte mére"))
        .or_else(|| input.strip_prefix("Carte Mère"))
        .map(|s| s.trim().to_string())
}

fn parse_case(input: &str) -> Option<String> {
    input.strip_prefix("Boîtier Gaming")
        .or_else(|| input.strip_prefix("Boîtier Gamer"))
        .or_else(|| input.strip_prefix("Boîtier"))
        .map(|s| s.trim().to_string())
}

fn parse_warranty(input: &str) -> Option<(u32, Option<String>)> {
    let caps = WARRANTY_RE.captures(input)?;

    let years = caps.get(1)?.as_str().parse::<u32>().ok()?;

    let extra = caps.get(2)
        .map(|m| m.as_str().trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            if s.starts_with("+ ") {
                s[2..].to_string()
            } else { s.to_string() }
        });

    Some((years, extra))
}

fn parse_os(input: &str) -> Option<String> {
    let lower = input.to_lowercase();
    Some(if lower.contains("windows") {
        "Windows 11"
    } else if lower.contains("macos") {
        "MacOS"
    } else {
        "FreeDos"
    }.to_string())
}

fn parse_psu(input: &str) -> Option<String> {
    input.strip_prefix("Boîte d'alimentation")
        .or_else(|| input.strip_prefix("Alimentation"))
        .map(|s| s.trim().to_string())
}

fn parse_cooler(input: &str) -> Option<String> {
    match input.to_lowercase().find("processeur") {
        Some(pos) => Some(input[pos + 10..].trim().to_string()),
        None => Some(input.to_string()),
    }
}