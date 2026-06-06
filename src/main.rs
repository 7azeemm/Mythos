pub mod web_scraper;
pub mod api;
pub mod utils;
pub mod ai;

use crate::api::server;
use crate::utils::logger::setup_logging;
use crate::utils::web_client::{WebClient, WebClientType};
use crate::web_scraper::manager::ProductManager;
use dotenv::dotenv;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use utils::database;

#[tokio::main]
async fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info,playwright_rs=off"); }
    
    println!("Starting...");
    dotenv().ok();
    setup_logging();

    // ai::run().await.unwrap();
    // return;

    WebClient::init().await;
    database::connect().await;
    ProductManager::schedule().await;

    server::run(3000).await.expect("Failed to start server");

    WebClient::cleanup().await.ok();
}