use crate::web_scraper::product::Product;
use crate::web_scraper::sites::{PAGE_CACHE, SITES};
use crate::web_scraper::specs::cache::SPECS_CACHE;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use crate::web_scraper::sections::Section;
use crate::web_scraper::updater::sync_products;

pub static TO_PRINT: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn print(text: &str) {
    TO_PRINT.lock().unwrap().insert(text.to_string());
}

pub fn schedule() {
    tokio::spawn(async move {
        loop {
            load_cache().await;

            let mut products = HashMap::default();

            for section in [Section::CPU] {
                for site in SITES.iter() {
                    println!("------- {} -------", site.name());
                    for (_, url) in site.config().sections.iter().filter(|(s, _)| *s == &section) {
                        println!("--- {} ({}) ---", section.to_str().to_uppercase(), url);
                        site.scrape(url, &section, &mut products).await;
                    }
                }
            }

            // Save to Cache
            save_cache(&*PAGE_CACHE.read().await).await.expect("Cache initialization failed");

            // if let Err(err) = sync_products(&products).await {
            //     eprintln!("Failed to sync products: {err}");
            // }
            //
            // SPECS_CACHE.write().await.initialize(products).await;

            println!("---");
            let mut sorted_items: Vec<String> = TO_PRINT.lock().unwrap().iter().cloned().collect();
            sorted_items.sort();
            for to_print in sorted_items {
                println!("{to_print}")
            }

            sleep(Duration::from_secs(3600)).await;
        }
    });
}

async fn load_cache() {
    let start_time = Instant::now();
    let Ok(data) = tokio::fs::read_to_string("pages_cache.json").await else {
        return;
    };

    let parsed: HashMap<String, HashMap<String, Product>> = serde_json::from_str(&data)
        .expect("Cache initialization failed");
    *PAGE_CACHE.write().await = parsed;

    println!("Loaded cache in {:.2?}", start_time.elapsed());
}

async fn save_cache(cache: &HashMap<String, HashMap<String, Product>>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(cache)?;
    tokio::fs::write("pages_cache.json", data).await?;
    Ok(())
}