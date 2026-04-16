use regex::Regex;
use crate::models::{Memory, RamType};
use crate::Product;

pub fn extract(product: &Product) -> Vec<String> {
    let mut parts: Vec<&str> = product.desc.split(" - ").collect();
    let is_setup = product.title.starts_with("Setup");

    let (case, monitor) = match parts.first().unwrap().contains("Écran") {
        true => {
            let monitor = parts.first().unwrap().to_string();
            parts.remove(0);
            (parts.first().unwrap(), Some(monitor))
        },
        false => (parts.first().unwrap(), None)
    };

    let cpu = parts.get(1).unwrap();
    let memory = parts.get(2).unwrap();

    let cpu_parsed = parse_cpu_spec(cpu).unwrap();

    vec![format!("{} ({},{})", cpu_parsed.0, cpu_parsed.1, cpu_parsed.2)]
}

fn parse_memory_spec(input: &str) -> Option<Memory> {
    let re = Regex::new(
        r"(?i)^mémoire\s+(\d+)\s*go\s*\(\s*(\d+)\s*[x×]\s*(\d+)\s*go\s*\)\s*(ddr\d)\s*$"
    ).ok()?;

    let caps = re.captures(input)?;

    Some(Memory {
        total_gb: caps.get(1)?.as_str().parse().ok()?,
        sticks: caps.get(2)?.as_str().parse().ok()?,
        per_stick_gb: caps.get(3)?.as_str().parse().ok()?,
        ram_type: RamType::parse(caps.get(4)?.as_str())?
    })
}

fn parse_cpu_spec(input: &str) -> Option<(String, String, String)> {
    let re = Regex::new(
        r"(?i)Processeur\s+(.*?)(?:,\s*\(|\s*,\s*).*?([\d.]+)\s*ghz.*?(\d+)\s*mo"
    ).ok()?;

    let caps = re.captures(input)?;

    let raw_name = caps.get(1)?.as_str();
    let name_clean = Regex::new(r"(?i)\s+\d+\s*(?:e|ème|eme)?\s+géné.*")
        .ok()?
        .replace(raw_name, "")
        .trim_end_matches(',')
        .trim()
        .to_string();

    let clock = caps.get(2)?.as_str().to_string();
    let cache = caps.get(3)?.as_str().to_string();

    Some((name_clean, clock, cache))
}