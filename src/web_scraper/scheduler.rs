use std::time::Duration;
use tokio::time::sleep;
use crate::parser::specs_cache;
use crate::web_scraper::fetcher::scrape_tunisianet;

pub fn schedule() {
    tokio::spawn(async move {
        loop {
            scrape_tunisianet().await;

            specs_cache::initialize_cache().await;
            sleep(Duration::from_secs(3600)).await;
        }
    });
}