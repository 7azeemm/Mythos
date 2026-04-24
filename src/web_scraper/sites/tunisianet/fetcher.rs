use crate::web_scraper::product::Product;
use crate::HTTP_CLIENT;
use chrono::Utc;
use scraper::element_ref::Text;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::time::Instant;
use crate::web_scraper::sites::tunisianet::{DESC_SEL, ID_RE, IMAGE_SEL, PRICE_SEL, PRODUCTS_SEL, PRODUCT_SEL, REF_SEL, STATUS_SEL, TITLE_SEL, SECTIONS, URL_SEL};

pub async fn scrape(products: &mut Vec<Product>) {
    for (section, url) in SECTIONS {
        scrape_section(&section.to_str(), products, url).await;
    }
}

async fn scrape_section(section: &str, products: &mut Vec<Product>, url: &str) {
    println!("Scraping {url}");
    let init_count = products.len();
    let start_time = Instant::now();

    let (page_count, first_page_count) = match fetch_first_page(section, products, url).await {
        Ok((count, first_page_count)) => (count, first_page_count),
        Err(err) => {
            eprintln!("Failed to fetch first page: {err}");
            return;
        }
    };
    
    println!("Found {page_count} pages");
    println!("Scraped 1 ({first_page_count})");

    for page in 2..page_count+1 {
        let document = match fetch_page(&format!("{url}?page={page}")).await {
            Ok(doc) => doc,
            Err(err) => {
                eprintln!("Failed to fetch page {page}: {err}");
                continue;
            }
        };
        match extract_products(section, products, document) {
            Ok(count) => println!("Scraped {page} ({count})"),
            Err(err) => eprintln!("Failed to extract products from page {page}: {err}")
        }
    }

    println!(
        "Scraped in {:.2?} ({} products)",
        start_time.elapsed(),
        products.len() - init_count
    );
}

async fn fetch_page(url: &str) -> Result<Html, Box<dyn Error>> {
    let body = HTTP_CLIENT.get(url).send().await?.text().await?;
    Ok(Html::parse_document(&body))
}

async fn fetch_first_page(section: &str, products: &mut Vec<Product>, url: &str) -> Result<(u32, usize), Box<dyn Error>> {
    let document = fetch_page(url).await?;

    let nav_sel = Selector::parse("nav.pagination ul.page-list")?;
    let page_sel = Selector::parse("li")?;

    let page_count = match document.select(&nav_sel).next() {
        None => 1,
        Some(nav) => {
            let elements = nav.select(&page_sel).collect::<Vec<ElementRef>>();
            let last_page = elements.get(elements.len() - 2).ok_or("last page button not found")?;
            last_page.text().collect::<Vec<_>>().join(" ").trim().parse::<u32>()?
        }
    };

    let count = extract_products(section, products, document)?;

    Ok((page_count, count))
}

fn extract_products(section: &str, products: &mut Vec<Product>, document: Html) -> Result<usize, Box<dyn Error>> {
    let list = document.select(&PRODUCTS_SEL).next().ok_or("products not found")?;
    let init_count = products.len();

    for product in list.select(&PRODUCT_SEL) {
        match extract_product(section, product) {
            Ok(product) => products.push(product),
            Err(err) => match product.select(&REF_SEL).next().and_then(|e| Some(extract_text(e.text()))) {
                Some(id) => eprintln!("Failed to extract product {id}: {err}"),
                None => eprintln!("Failed to extract product: {err}")
            }
        }
    }

    Ok(products.len() - init_count)
}

fn extract_product(section: &str, product: ElementRef) -> Result<Product, Box<dyn Error>> {
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
        section: section.to_string(),
        source: "Tunisianet".to_string(),
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