use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct CycleReport {
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,

    pub scrape: Vec<SectionReport>,
    pub parse: Vec<ParseError>,
    pub update: UpdateReport,

    pub added_items: Vec<Product>,
    pub edited_items: Vec<(Product, Vec<Value>)>,
    pub removed_items: Vec<Product>,
}

impl CycleReport {
    pub fn new() -> Self {
        Self {
            started_at: Utc::now(),
            completed_at: DateTime::default(),
            duration: Duration::default(),
            scrape: Vec::new(),
            parse: Vec::new(),
            update: UpdateReport::default(),
            added_items: Vec::new(),
            edited_items: Vec::new(),
            removed_items: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionReport {
    pub section: Section,
    pub total_products: usize,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    pub sites: Vec<SiteReport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteReport {
    pub site: String,
    pub page_count: usize,
    pub total_products: usize,
    pub pages: Vec<PageReport>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageReport {
    pub url: String,
    pub products: usize,
    pub errors: Vec<ScrapeError>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeError {
    pub error: ScrapeErrorKind,
    pub section: Section,
    pub site: String,
    pub url: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ScrapeErrorKind {
    FetchFailed(String),
    ParseFailed(String),
    ZeroProducts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub error: ParseErrorKind,
    pub product: Product,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq)]
pub enum ParseErrorKind {
    NoSectionMatched,
    NotInDataset
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateReport {
    pub added: usize,
    pub edited: usize,
    pub removed: usize,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    pub errors: Vec<UpdateError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateError {
    pub error: UpdateErrorKind,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum UpdateErrorKind {
    FetchMissingProducts,
    InsertToArchive,
    InsertProducts,
    DeleteProducts,
    SelectProducts,
    UpdateProduct,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KnownEvents {
    scrape: Mutex<HashMap<u64, (EventRecord, ScrapeError)>>,
    parse: Mutex<HashMap<u64, (EventRecord, ParseError)>>,
    update: Mutex<HashMap<u64, (EventRecord, UpdateError)>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventRecord {
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    seen_count: usize,
}

impl KnownEvents {
    pub fn scrape_error(&self, error: ScrapeError) {
        let mut scrape = self.scrape.lock().unwrap();
        let fingerprint = scrape_error_fingerprint(&error);
        self.should_notify::<ScrapeError>(&mut scrape, fingerprint, error);
    }

    pub fn parse_error(&self, error: ParseError) {
        let mut parse = self.parse.lock().unwrap();
        let fingerprint = parse_error_fingerprint(&error);
        self.should_notify::<ParseError>(&mut parse, fingerprint, error);
    }

    pub fn update_error(&self, error: UpdateError) {
        print!("UpdateError: {:?}: {}", error.error, error.message);
        let mut update = self.update.lock().unwrap();
        let fingerprint = update_error_fingerprint(&error);
        self.should_notify::<UpdateError>(&mut update, fingerprint, error);
    }

    fn should_notify<T: Debug + Clone>(&self, mut map: &mut HashMap<u64, (EventRecord, T)>, fingerprint: u64, error: T) {
        match map.get_mut(&fingerprint) {
            Some((record, _)) => {
                record.last_seen = Utc::now();
                record.seen_count += 1;
            }
            None => {
                let now = Utc::now();
                map.insert(fingerprint, (EventRecord {
                    first_seen: now,
                    last_seen: now,
                    seen_count: 1,
                }, error.clone()));

                // Notify
                // println!("Notification: {:#?}", error);
            }
        }
    }
}

fn scrape_error_fingerprint(error: &ScrapeError) -> u64 {
    let mut h = DefaultHasher::new();
    error.error.hash(&mut h);
    error.section.hash(&mut h);
    error.site.hash(&mut h);
    error.url.hash(&mut h);
    h.finish()
}

fn parse_error_fingerprint(error: &ParseError) -> u64 {
    let mut h = DefaultHasher::new();
    error.error.hash(&mut h);
    error.product.url.hash(&mut h);
    h.finish()
}

fn update_error_fingerprint(error: &UpdateError) -> u64 {
    let mut h = DefaultHasher::new();
    error.error.hash(&mut h);
    error.message.hash(&mut h);
    h.finish()
}