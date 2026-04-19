pub mod extractor;
pub mod models;
pub mod dataset;
pub mod site_scraper;
pub mod database;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;
use dotenv::dotenv;
use futures::StreamExt;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new().build().expect("Failed to build HTTP client")
});

#[tokio::main]
async fn main() {
    println!("Starting...");
    dotenv().ok();
    database::connect().await;
    dataset::load_datasets();
    site_scraper::schedule();

    sleep(Duration::from_hours(1)).await;
}

fn save_json<T: Serialize>(path: &str, value: &T) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json)?;
    Ok(())
}

fn load_json<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let value = serde_json::from_str::<T>(&raw)?;
    Ok(value)
}

