use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::time::sleep;
use crate::web_scraper::specs_cache;
use crate::utils::database::get_db_pool;
use crate::web_scraper::sites::tunisianet;
use crate::web_scraper::specs_cache::{SpecsCache, SPECS_CACHE};
use crate::web_scraper::updater::sync_products;

pub fn schedule() {
    tokio::spawn(async move {
        loop {
            let mut products = Vec::new();

            tunisianet::fetcher::scrape(&mut products).await;

            products.sort_by(|a, b| a.id.cmp(&b.id));
            products.dedup_by(|a, b| a.id == b.id);

            if let Err(err) = sync_products(get_db_pool(), products).await {
                eprintln!("Failed to sync products: {err}");
            }
            
            SPECS_CACHE.write().await.initialize().await;
            sleep(Duration::from_secs(3600)).await;
        }
    });
}