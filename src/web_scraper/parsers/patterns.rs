use once_cell::sync::Lazy;
use regex::Regex;

pub(super) static CPU_CLEANUP_RE: Lazy<Regex> = Lazy::new(|| Regex::new(
    r"(?i)(?:type\s+de\s+)?processeur\s*:?\s*|puce\s*|\(.*|\s+\d+\s*(?:e|ème|éme|eme)?\s+géné.*"
).unwrap());

static GPU_PATTERNS: &[&str] = &[
    "AMD", "ASUS", "NVIDIA", "GIGABYTE", "MSI",
    "INNO3D", "TWIN", "VENTUS", "WINDFORCE", "Édition",
    "TRIO", "GAMING", "WHITE", "SAPPHIRE PULSE", "XFX Swift",
    "ZOTAC", "EDGE", "Dual", "INSPIRE", "SHADOW", "BULK", "PLUS",
    "X2", "OC", "V2", "V3", "XS", "2X", "3X", ",",
];

pub(super) static GPU_PREFIX_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)(?:c\s*arte|chipset)\s+graphique\s*:?\s*").unwrap());
pub(super) static GPU_PARENTHESES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\([^)]*\)").unwrap());
pub(super) static GPU_INTEGREE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)int[eéèê]gr[eéèê]e?s?").unwrap());
pub(super) static GPU_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(&format!(r"(?i){}", GPU_PATTERNS.join("|"))).unwrap());
pub(super) static GPU_VRAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(\d+)\s*(?:go|gb|g)\b.*$").unwrap());
pub(super) static GPU_VRAM_TYPE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(g?ddr\d[a-z]?)\b").unwrap());
pub(super) static GPU_MULTI_SPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

pub(super) static STORAGE_SIZE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(\d+)\s*(go|gb|t|to|tb)\b").unwrap());

pub(super) static RAM_SIZE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(\d+)\s*(?:go|gb)\b").unwrap());
pub(super) static RAM_STICKS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(\d+)\s*x\b").unwrap());
pub(super) static RAM_TYPE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b((?:lp)?ddr\dx?)\b").unwrap());
pub(super) static RAM_SPEED_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(\d{4})\b").unwrap());

pub(super) static WARRANTY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\b(\d+)\s*ans?\b(.*)").unwrap());
