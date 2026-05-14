use crate::web_scraper::sections::Section;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Encode, FromRow, Postgres, Type};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    #[serde(skip)]
    pub id: String,
    #[serde(skip)]
    pub name: String,
    pub url: String,
    pub title: String,
    pub site: String,
    pub original_section: Section,
    pub sections: Vec<Section>,
    pub description: Option<String>,
    pub image: String,
    pub status: ProductStatus,
    pub price: i32,
    pub old_price: Option<i32>,
    #[serde(skip)]
    pub specs: Value,
    pub history: Value,
    #[sqlx(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub removed_at: Option<DateTime<Utc>>,
    pub added_at: DateTime<Utc>,
}

impl Product {
    pub fn new(site: &str, url: String, title: String, section: Section,
               description: Option<String>, image: String, status: ProductStatus,
               price: i32, old_price: Option<i32>) -> Result<Self, String> {

        let original_section = section;
        let sections = match section.parent() {
            Some(parent) => vec![parent, section],
            None => vec![section]
        };

        Ok(Self {
            id: "".to_string(),
            name: "".to_string(),
            url,
            title,
            site: site.to_string(),
            original_section,
            sections,
            description,
            image,
            status,
            price,
            old_price,
            specs: Value::default(),
            history: Default::default(),
            updated_at: None,
            removed_at: None,
            added_at: Utc::now()
        })
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
            (Self::InStock, vec!["instock", "en stock", "in stock", "ajouter"]),
            (Self::OutOfStock, vec!["outofstock", "hors stock", "epuisé", "rupture de stock"]),
            (Self::OnArrive, vec!["onarrive", "en arrivage"]),
            (Self::OnRequest, vec!["onrequest", "sur commande"]),
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

impl sqlx::Type<Postgres> for ProductStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::Type<Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, Postgres> for ProductStatus {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as sqlx::Encode<'_, Postgres>>::encode_by_ref(&self.to_string().as_str(), buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for ProductStatus {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <String as sqlx::Decode<'r, Postgres>>::decode(value)?;
        ProductStatus::from_str(&s).map_err(|e| e.into())
    }
}

impl sqlx::postgres::PgHasArrayType for ProductStatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}