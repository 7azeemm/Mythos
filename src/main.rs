pub mod web_scraper;
pub mod api;
pub mod utils;

use crate::api::server;
use crate::utils::logger::setup_logging;
use crate::utils::web_client::WebClient;
use dotenv::dotenv;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use utils::{database, dataset};

#[tokio::main]
async fn main() {
    unsafe { std::env::set_var("RUST_LOG", "warn,playwright_rs=off"); }
    
    println!("Starting...");
    dotenv().ok();
    setup_logging();
    WebClient::init().await;
    database::connect().await;
    dataset::load_datasets();
    web_scraper::scheduler::schedule();

    tokio::spawn(async {
        server::run(3000).await.expect("Failed to start server")
    });

    tokio::signal::ctrl_c().await.ok();

    WebClient::cleanup().await.ok();
}