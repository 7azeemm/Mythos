pub mod extractor;
pub mod models;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use scraper::{ElementRef, Html, Selector};
use scraper::element_ref::Text;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use crate::extractor::extract;

const URL: &str = "https://www.tunisianet.com.tn/682-pc-de-bureau-gamer";
const MAX_CONCURRENT_REQUESTS: usize = 10;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new().build().expect("Failed to build HTTP client")
});

#[tokio::main]
async fn main() {
    println!("Starting...");

    let mut pages: Vec<PageData> = load_json("data.json").unwrap();
    pages.sort_by(|a, b| a.page.cmp(&b.page));

    let mut parts = Vec::new();

    for page in pages {
        for product in page.products {
            parts.append(&mut extract(&product));
        }
    }

    parts.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    parts.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());

    for part in parts {
        println!("{:?}", part);
    }

    // let page_count = get_page_count().await.expect("Failed to get page count");
    // println!("Found {page_count} pages");
    //
    // let mut tasks = FuturesUnordered::new();
    // let mut next_page = 1;
    // let mut all_pages = Vec::new();
    //
    // while next_page <= page_count || !tasks.is_empty() {
    //     while tasks.len() < MAX_CONCURRENT_REQUESTS && next_page <= page_count {
    //         let page = next_page;
    //
    //         tasks.push(async move {
    //             (page, scrape_page(page).await)
    //         });
    //
    //         next_page += 1;
    //     }
    //
    //     if let Some((page, result)) = tasks.next().await {
    //         match result {
    //             Ok(products) => {
    //                 println!("Page {} done ({} products)", page, products.len());
    //                 all_pages.push(PageData { page, products });
    //             }
    //             Err(e) => {
    //                 eprintln!("Page {} failed: {:?}", page, e);
    //             }
    //         }
    //     }
    // }
    //
    // save_json("data.json", &all_pages).unwrap();
}

async fn get_page_count() -> Result<u32, Box<dyn Error>> {
    let body = HTTP_CLIENT.get(URL)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);

    let nav_sel = Selector::parse("nav.pagination ul.page-list")?;
    let page_sel = Selector::parse("li")?;

    let nav = document.select(&nav_sel).next().ok_or("navigator not found")?;
    let elements = nav.select(&page_sel).collect::<Vec<ElementRef>>();
    let last_page = elements.get(elements.len() - 2).unwrap();

    Ok(last_page.text().collect::<Vec<_>>().join(" ").trim().parse::<u32>()?)
}

async fn scrape_page(page: u32) -> Result<Vec<Product>, Box<dyn Error>> {
    println!("Fetching page {page}...");
    let body = HTTP_CLIENT.get(&format!("{URL}?page={page}"))
        .send()
        .await?
        .text()
        .await?;
    
    let document = Html::parse_document(&body);
    
    let products_sel = Selector::parse("div.products")?;
    let product_sel = Selector::parse("div.item-product")?;
    let image_sel = Selector::parse("a.product-thumbnail img[data-full-size-image-url]")?;
    let title_sel = Selector::parse("h2.product-title")?;
    let url_sel = Selector::parse("h2.product-title a[href]")?;
    let ref_sel = Selector::parse("span.product-reference")?;
    let desc_sel = Selector::parse(r#"div.product-description div[itemprop="description"]"#)?;
    let status_sel = Selector::parse("div#stock_availability")?;
    let price_sel = Selector::parse("span.price")?;

    let products = document.select(&products_sel).next().unwrap();
    let mut products_list = Vec::new();

    for product in products.select(&product_sel) {
        let title = product.select(&title_sel).next().ok_or("title not found")?;
        let url = product.select(&url_sel).next().ok_or("url not found")?;
        let image = product.select(&image_sel).next().ok_or("image not found")?;
        let product_ref = product.select(&ref_sel).next().ok_or("product_ref not found")?;
        let desc = product.select(&desc_sel).next().ok_or("desc not found")?;
        let status = product.select(&status_sel).next().ok_or("status not found")?;
        let price = product.select(&price_sel).next().ok_or("price not found")?;

        let image_url = image
            .value()
            .attr("data-full-size-image-url")
            .or_else(|| image.value().attr("src"))
            .ok_or("image url not found")?
            .to_string();

        let url = url
            .value()
            .attr("href")
            .ok_or("product url not found")?
            .to_string();

        let title = extract_text(title.text());
        let product_ref = extract_text(product_ref.text());
        let desc = extract_text(desc.text());
        let status = extract_text(status.text());
        let price = extract_text(price.text());

        products_list.push(Product {
            title,
            url,
            image_url,
            product_ref,
            desc,
            status,
            price,
        });
    }

    Ok(products_list)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub title: String,
    pub url: String,
    pub image_url: String,
    pub product_ref: String,
    pub desc: String,
    pub status: String,
    pub price: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PageData {
    page: u32,
    products: Vec<Product>,
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

fn extract_text(text: Text) -> String {
    text.collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}