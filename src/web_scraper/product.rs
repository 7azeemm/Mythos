use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, FromRow};
use std::fmt::Display;
use std::str::FromStr;

//TODO: Enum Status maybe, to prevent bugs in the future caused by site changements of text

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub url: String,
    pub title: String,
    pub source: String,
    pub sections: Vec<String>,
    pub description: Vec<String>,
    pub image: String,
    pub in_stock: bool,
    pub regular_price: Option<i32>,
    pub price: i32,
    pub history: Value,
    pub added_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}