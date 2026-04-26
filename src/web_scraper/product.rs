use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, FromRow};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub p_ref: String,
    pub section: String,
    pub source: String,
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