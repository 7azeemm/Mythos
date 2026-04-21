use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use tokio::sync::Mutex;
use crate::web_scraper::product::Product;

static MISSING_MAP: Lazy<Mutex<HashMap<String, u8>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn sync_products(pool: &PgPool, scraped: Vec<Product>) -> Result<(), Box<dyn Error>> {
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

        let price_changed = product.price != existing.price;
        let status_changed = product.status != existing.status;
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
                push_change(&mut history, &existing.id, "price", existing.price, product.price);
            }
            if status_changed {
                push_change(&mut history, &existing.id, "status", &existing.status, &product.status);
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
                product.price,
                product.status,
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
        product.status,
        product.price,
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