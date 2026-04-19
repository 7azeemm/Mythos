use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::time::sleep;
use scraper::{ElementRef, Html, Selector};
use scraper::element_ref::Text;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use tokio::sync::Mutex;
use crate::{HTTP_CLIENT};
use crate::database::get_db_pool;
use crate::models::PCSpecs;

const URL: &str = "https://www.tunisianet.com.tn/682-pc-de-bureau-gamer";

static PRODUCTS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.products").unwrap());
static PRODUCT_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div.item-product").unwrap());
static TITLE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title").unwrap());
static REF_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.product-reference").unwrap());
static URL_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("h2.product-title a[href]").unwrap());
static IMAGE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("a.product-thumbnail img[data-full-size-image-url]").unwrap());
static DESC_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse(r#"div.product-description div[itemprop="description"]"#).unwrap());
static STATUS_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("div#stock_availability").unwrap());
static PRICE_SEL: Lazy<Selector> = Lazy::new(|| Selector::parse("span.price").unwrap());

static ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/(\d+)-").unwrap());

static MISSING_MAP: Lazy<Mutex<HashMap<String, u8>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub p_ref: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub status: StockStatus,
    pub price: u32,
    pub history: Value,
    pub added_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProductSpecs {
    PC(PCSpecs),
    Unknown
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StockStatus {
    InStock,
    OutOfStock,
}

impl FromStr for StockStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en stock" => Ok(StockStatus::InStock),
            _ => Err(format!("Unknown stock status: {}", s)),
        }
    }
}

impl Display for StockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockStatus::InStock => write!(f, "In Stock"),
            StockStatus::OutOfStock => write!(f, "Out of Stock"),
        }
    }
}

pub fn schedule() {
    tokio::spawn(async move {
        loop {
            if let Err(err) = scrape_tunisianet().await {
                eprintln!("Failed to scrape tunisianet: {}", err);
            }
            sleep(Duration::from_secs(3600)).await;
        }
    });
}

pub async fn scrape_tunisianet() -> Result<(), Box<dyn Error>> {
    println!("Scraping Tunisianet...");
    let start_time = Instant::now();

    let (page_count, mut products) = fetch_first_page().await?;
    println!("Found {page_count} pages");
    println!("Scraped 1 ({})", products.len());

    for page in 2..page_count+1 {
        let document = match fetch_page(&format!("{URL}?page={page}")).await {
            Ok(doc) => doc,
            Err(err) => {
                eprintln!("Failed to fetch page {page}: {err}");
                continue;
            }
        };
        match extract_products(document) {
            Ok(list) => {
                println!("Scraped {page} ({})", list.len());
                products.extend(list);
            },
            Err(err) => eprintln!("Failed to extract products from page {page}: {err}")
        }
    }

    println!(
        "Scraped Tunisianet in {:.2?} ({} page, {} products)",
        start_time.elapsed(),
        page_count,
        products.len()
    );

    sync_products(get_db_pool(), products).await?;

    Ok(())
}

async fn fetch_page(url: &str) -> Result<Html, Box<dyn Error>> {
    let body = HTTP_CLIENT.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&body))
}

async fn fetch_first_page() -> Result<(u32, Vec<Product>), Box<dyn Error>> {
    let document = fetch_page(URL).await?;

    let nav_sel = Selector::parse("nav.pagination ul.page-list")?;
    let page_sel = Selector::parse("li")?;

    let nav = document.select(&nav_sel).next().ok_or("navigator not found")?;
    let elements = nav.select(&page_sel).collect::<Vec<ElementRef>>();
    let last_page = elements.get(elements.len() - 2).ok_or("last page button not found")?;
    let page_count = last_page.text().collect::<Vec<_>>().join(" ").trim().parse::<u32>()?;
    let products = extract_products(document)?;

    Ok((page_count, products))
}

fn extract_products(document: Html) -> Result<Vec<Product>, Box<dyn Error>> {
    let list = document.select(&PRODUCTS_SEL).next().ok_or("products not found")?;
    let mut products = Vec::new();

    for product in list.select(&PRODUCT_SEL) {
        match extract_product(product) {
            Ok(product) => products.push(product),
            Err(err) => match product.select(&REF_SEL).next().and_then(|e| Some(extract_text(e.text()))) {
                Some(id) => eprintln!("Failed to extract product {id}: {err}"),
                None => eprintln!("Failed to extract product: {err}")
            }
        }
    }

    Ok(products)
}

fn extract_product(product: ElementRef) -> Result<Product, Box<dyn Error>> {
    let title = product.select(&TITLE_SEL).next().ok_or("title not found")?;
    let url = product.select(&URL_SEL).next().ok_or("url not found")?;
    let image = product.select(&IMAGE_SEL).next().ok_or("image not found")?;
    let p_ref = product.select(&REF_SEL).next().ok_or("ref not found")?;
    let desc = product.select(&DESC_SEL).next().ok_or("desc not found")?;
    let status = product.select(&STATUS_SEL).next().ok_or("status not found")?;
    let price = product.select(&PRICE_SEL).next().ok_or("price not found")?;

    let image = image
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

    let mut p_ref = extract_text(p_ref.text());
    let title = extract_text(title.text());
    let description = extract_text(desc.text());
    let status = extract_text(status.text());
    let price = extract_text(price.text());

    let id: String = ID_RE.captures(&url)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or::<String>(format!("failed to extract id from `{url}`").into())?;

    let status = StockStatus::from_str(&status)?;
    let price = price.replace("DT", "").replace(" ", "").replace(",", "").parse::<u32>()?;

    Ok(Product {
        id,
        title,
        p_ref,
        url,
        image,
        description,
        status,
        price,
        history: Value::Array(vec![]),
        added_at: None,
        updated_at: None,
        created_at: Utc::now(),
    })
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

async fn sync_products(pool: &PgPool, scraped: Vec<Product>) -> Result<(), Box<dyn Error>> {
    let ids: Vec<_> = scraped.iter().map(|p| p.id.clone()).collect();
    let ids_set = ids.iter().collect::<HashSet<_>>();
    remove_old_products(pool, &ids_set).await?;

    let db_products = sqlx::query!(
        r#"
        SELECT id, p_ref, title, description, image, price, status, history
        FROM products
        WHERE id = ANY($1)
        "#,
        &ids
    )
        .fetch_all(pool)
        .await?;

    let mut existings = HashMap::new();
    for p in db_products {
        existings.insert(p.id.clone(), p);
    }

    for product in scraped {
        let existing = match existings.get(&product.id) {
            Some(p) => p,
            None => {
                insert_new_product(pool, &product, &product.id).await?;
                println!("NEW: {}", product.id);
                continue;
            }
        };

        if existing.title != product.title {
            eprintln!(
                r#"FOUND DUPE:
                - id: {} | ref: {} | title: {}
                - id: {} | ref: {} | title: {}
                "#,
                existing.id, existing.p_ref, existing.title,
                product.id, product.p_ref, product.title
            );
            continue;
        }

        let status = product.status.to_string();
        let price_changed = product.price != existing.price as u32;
        let status_changed = status != existing.status;
        let desc_changed = product.description != existing.description;
        let title_changed = product.title != existing.title;
        let image_changed = product.image != existing.image;

        if price_changed || status_changed || desc_changed || title_changed || image_changed {
            let mut history = existing
                .history
                .as_array()
                .cloned()
                .unwrap_or_default();

            if price_changed {
                push_change(&mut history, &existing.id, "status", existing.price, product.price as i32);
            }
            if status_changed {
                push_change(&mut history, &existing.id, "price", &existing.status, &status);
            }
            if desc_changed {
                push_change(&mut history, &existing.id, "description", &existing.description, &product.description);
            }
            if title_changed {
                push_change(&mut history, &existing.id, "title", &existing.title, &product.title);
            }
            if image_changed {
                push_change(&mut history, &existing.id, "image", &existing.image, &product.image);
            }

            let new_history = Value::Array(history);

            sqlx::query!(
                r#"
                UPDATE products
                SET title = $1,
                    price = $2,
                    status = $3,
                    description = $4,
                    image = $5,
                    history = $6,
                    updated_at = $7
                WHERE id = $8
                "#,
                product.title,
                product.price as i32,
                status,
                product.description,
                product.image,
                new_history,
                Utc::now(),
                existing.id
            )
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

async fn remove_old_products(pool: &PgPool, ids: &HashSet<&String>) -> Result<(), sqlx::Error> {
    let db_products = sqlx::query!(
        r#"
        SELECT id FROM products
        "#
    )
        .fetch_all(pool)
        .await?;

    let mut map = MISSING_MAP.lock().await;

    for db in db_products {
        if !ids.contains(&db.id) {
            let count = map.entry(db.id.clone()).or_insert(0);
            *count += 1;

            if *count >= 1 {
                sqlx::query!(
                    r#"
                    INSERT INTO products_archive
                    (id, p_ref, title, description, url, image, status, price, history,
                     added_at, removed_at, updated_at, created_at)
                    SELECT id, p_ref, title, description, url, image, status, price, history,
                     added_at, $2, updated_at, created_at
                    FROM products WHERE id = $1
                    "#,
                    db.id,
                    Utc::now()
                )
                    .execute(pool)
                    .await?;

                sqlx::query!(
                    r#"
                    DELETE FROM products WHERE id = $1
                    "#,
                    db.id
                )
                    .execute(pool)
                    .await?;

                map.remove(&db.id);

                println!("REMOVED: {}", db.id);
            }
        }
    }

    Ok(())
}

async fn insert_new_product(pool: &PgPool, product: &Product, id: &str) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
        INSERT INTO products
        (id, p_ref, title, description, url, image, status, price,
        history, added_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        "#,
        id,
        product.p_ref,
        product.title,
        product.description,
        product.url,
        product.image,
        product.status.to_string(),
        product.price as i32,
        Value::Array(vec![]),
        Utc::now()
    )
        .execute(pool)
        .await?;
    Ok(())
}

fn push_change<T: Serialize + Display>(history: &mut Vec<Value>, id: &str, field: &str, old: T, new: T) {
    history.push(json!({
        "field": field,
        "old": old,
        "new": new,
        "ts": Utc::now()
    }));
    println!(
        "EDIT `{}` in {}: {} => {}",
        field, id, old, new
    );
}