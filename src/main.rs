pub mod web_scraper;
pub mod api;
pub mod utils;

use std::error::Error;
use dotenv::dotenv;
use futures::StreamExt;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use utils::{database, dataset};
use crate::api::server;
use crate::utils::logger::setup_logging;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new().build().expect("Failed to build HTTP client")
});

#[tokio::main]
async fn main() {
    println!("Starting...");
    dotenv().ok();
    setup_logging();
    database::connect().await;
    dataset::load_datasets();
    web_scraper::scheduler::schedule();

    server::run(3000).await.expect("Failed to start the server");
}