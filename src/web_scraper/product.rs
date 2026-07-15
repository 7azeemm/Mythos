use std::collections::HashMap;
use crate::web_scraper::sections::Section;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub url: String,
    pub title: String,
    pub site: String,
    pub section: Section,
    pub description: Option<String>,
    pub image: String,
    pub status: ProductStatus,
    pub price: i32,
    pub old_price: Option<i32>,
    #[serde(skip_deserializing)]
    pub filter_ids: HashMap<String, String>,
    #[serde(skip_deserializing)]
    pub components: HashMap<String, String>,
    pub history: Value,
    pub updated_at: Option<DateTime<Utc>>,
    pub removed_at: Option<DateTime<Utc>>,
    pub added_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        site: &str, url: String, title: String, section: Section, description: Option<String>,
        image: String, status: ProductStatus, price: i32, old_price: Option<i32>
    ) -> Result<Self, String> {

        let normalized_url = url.trim_end_matches("/").trim_end_matches(".html").to_lowercase();
        let str = format!("{}:{normalized_url}", site.to_lowercase());
        let mut hasher = DefaultHasher::new();
        str.hash(&mut hasher);
        let id = hasher.finish().to_string();
        
        Ok(Self {
            id,
            url,
            title,
            site: site.to_string(),
            section,
            description,
            image,
            status,
            price,
            old_price,
            filter_ids: Default::default(),
            components: Default::default(),
            history: Default::default(),
            updated_at: None,
            removed_at: None,
            added_at: Utc::now()
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Specs(Map<String, Value>);

impl Specs {
    pub fn set(&mut self, key: &str, value: impl Into<Value>) {
        self.0.insert(key.to_string(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.0.get(key)?.as_str()
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductStatus {
    InStock,
    OutOfStock,
    OnArrive,
    OnRequest
}

impl Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductStatus::InStock => write!(f, "InStock"),
            ProductStatus::OutOfStock => write!(f, "OutOfStock"),
            ProductStatus::OnArrive => write!(f, "OnArrive"),
            ProductStatus::OnRequest => write!(f, "OnRequest"),
        }
    }
}

impl FromStr for ProductStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        let statuses = vec![
            (Self::InStock, vec!["instock", "en stock", "in stock", "ajouter", "disponible", "sale"]),
            (Self::OutOfStock, vec!["outofstock", "hors stock", "epuisé", "épuisé", "rupture de stock"]),
            (Self::OnArrive, vec!["onarrive", "en arrivage", "arriving"]),
            (Self::OnRequest, vec!["onrequest", "sur commande", "surcommande"]),
        ];

        for (status, keys) in statuses {
            for k in keys {
                if lower.contains(k) {
                    return Ok(status)
                }
            }
        }

        Err(format!("Unknown Product Status: {}", s))
    }
}