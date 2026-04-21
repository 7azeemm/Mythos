use crate::utils::database::get_db_pool;
use crate::web_scraper::updater;
use crate::HTTP_CLIENT;
use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::element_ref::Text;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use crate::web_scraper::product::Product;

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

    updater::sync_products(get_db_pool(), products).await?;

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

    let price = price.replace("DT", "")
        .replace(" ", "")
        .replace(",", "")
        .parse::<i32>()?;

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