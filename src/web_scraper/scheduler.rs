use crate::utils::database::get_db_pool;
use crate::web_scraper::product::Product;
use crate::web_scraper::sites::tunisianet;
use crate::web_scraper::sites::tunisianet::fetcher::PAGE_CACHE;
use crate::web_scraper::specs_cache::SPECS_CACHE;
use crate::web_scraper::updater::sync_products;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub static TO_PRINT: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn schedule() {
    tokio::spawn(async move {
        loop {
            let start_time = Instant::now();
            load_cache().await.expect("Cache initialization failed");
            println!("Loaded cache in {:.2?}", start_time.elapsed());

            let mut products = Vec::new();

            tunisianet::fetcher::scrape(&mut products).await;

            save_cache(&*PAGE_CACHE.read().await).await.expect("Cache initialization failed");

            products.sort_by(|a, b| a.id.cmp(&b.id));
            products.dedup_by(|a, b| a.id == b.id);

            if let Err(err) = sync_products(get_db_pool(), products).await {
                eprintln!("Failed to sync products: {err}");
            }
            
            SPECS_CACHE.write().await.initialize().await;

            println!("------");
            let mut sorted_items: Vec<String> = TO_PRINT.lock().unwrap().iter().cloned().collect();
            sorted_items.sort();
            for to_print in sorted_items {
                println!("{to_print}")
            }

            sleep(Duration::from_secs(3600)).await;
        }
    });
}

async fn load_cache() -> Result<(), Box<dyn Error>> {
    let data = match tokio::fs::read_to_string("pages_cache.json").await {
        Ok(content) => content,
        Err(_) => return Ok(()),
    };

    let parsed: HashMap<String, Vec<Product>> = serde_json::from_str(&data)?;

    let mut cache = PAGE_CACHE.write().await;
    *cache = parsed;

    Ok(())
}

async fn save_cache(cache: &HashMap<String, Vec<Product>>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(cache)?;
    tokio::fs::write("pages_cache.json", data).await?;
    Ok(())
}