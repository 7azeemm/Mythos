use std::fmt::Display;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, FromRow, Type};
use crate::parser::specs::PCSpecs;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub p_ref: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub status: String,
    pub price: i32,
    pub history: Value,
    pub added_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductSpecs {
    PC(PCSpecs),
    Unknown
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
pub enum StockStatus {
    InStock,
    OutOfStock,
}

impl FromStr for StockStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en stock" => Ok(StockStatus::InStock),
            _ => Err(format!("Unknown stock status: {}", s)),
        }
    }
}

impl Display for StockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockStatus::InStock => write!(f, "In Stock"),
            StockStatus::OutOfStock => write!(f, "Out of Stock"),
        }
    }
}