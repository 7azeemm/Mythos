pub mod web_scraper;
pub mod api;
pub mod utils;
pub mod storage;

use crate::api::server;
use crate::storage::ProductStorage;
use crate::utils::logger::setup_logging;
use crate::utils::web_client::WebClient;
use crate::web_scraper::manager::ProductManager;
use crate::web_scraper::sections::SectionConfig;
use dotenv::dotenv;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::utils::serde_ext::JsonExt;

#[tokio::main]
async fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info,playwright_rs=off"); }

    println!("Starting...");
    dotenv().ok();
    setup_logging();
    WebClient::init().await;

    // let data = FileLoader::load_csv("config/datasets/GPU-chipsets.csv").await.unwrap();
    // let mut map = HashMap::new();
    // for record in data {
    //     let mut name = record.get_str("name").unwrap().to_string();
    //     if let Some(memory) = record.get_str("memory_size") {
    //         if !memory.is_empty() {
    //             name.push_str(&format!(" {memory}GB"));
    //         }
    //     }
    //     map.insert(name, record);
    // }
    // FileLoader::save_to_file("config/datasets/GPU-chipsets.json", &map).await.unwrap();

    SectionConfig::load().await;
    ProductStorage::load().await;
    ProductManager::schedule().await;

    server::run(3000).await.expect("Failed to start server");

    WebClient::cleanup().await.ok();
}