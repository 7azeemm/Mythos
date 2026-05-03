use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use tokio::sync::Mutex;
use crate::utils::database::get_db_pool;
use crate::web_scraper::product::Product;

const BATCH_SIZE: usize = 100;
const UNSEEN_TIMES_TO_ARCHIVE: u8 = 1;

static MISSING_MAP: Lazy<Mutex<HashMap<String, u8>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn sync_products(products: &Vec<Product>) -> Result<(), Box<dyn Error>> {
    let pool = get_db_pool();
    let ids: Vec<_> = products.iter().map(|p| p.id.clone()).collect();
    let ids_set = ids.iter().collect::<HashSet<_>>();

    archive_missing_products(pool, &ids_set).await?;

    let db_products = sqlx::query!(
        r#"
        SELECT id, p_ref, section, source, title, description, image, price, status, history
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

    let mut new_products = Vec::new();

    // for product in products {
    //     let existing = match existings.get(&product.id) {
    //         Some(p) => p,
    //         None => {
    //             new_products.push(product);
    //             continue;
    //         }
    //     };
    //
    //     if existing.title != product.title {
    //         eprintln!(
    //             r#"FOUND DUPE:
    //             - id: {} | ref: {} | title: {}
    //             - id: {} | ref: {} | title: {}
    //             "#,
    //             existing.id, existing.p_ref, existing.title,
    //             product.id, product.p_ref, product.title
    //         );
    //         continue;
    //     }
    //
    //     let price_changed = product.price != existing.price;
    //     // let status_changed = product.status != existing.status;
    //     // let desc_changed = product.description != existing.description;
    //     let title_changed = product.title != existing.title;
    //     let image_changed = product.image != existing.image;
    //
    //     if price_changed || status_changed || desc_changed || title_changed || image_changed {
    //         let mut history = existing
    //             .history
    //             .as_array()
    //             .cloned()
    //             .unwrap_or_default();
    //
    //         if price_changed {
    //             push_change(&mut history, &existing.id, "price", existing.price, product.price);
    //         }
    //         if status_changed {
    //             push_change(&mut history, &existing.id, "status", &existing.status, &product.status);
    //         }
    //         if desc_changed {
    //             push_change(&mut history, &existing.id, "description", &existing.description, &product.description);
    //         }
    //         if title_changed {
    //             push_change(&mut history, &existing.id, "title", &existing.title, &product.title);
    //         }
    //         if image_changed {
    //             push_change(&mut history, &existing.id, "image", &existing.image, &product.image);
    //         }
    //
    //         sqlx::query!(
    //             r#"
    //             UPDATE products
    //             SET title = $1,
    //                 price = $2,
    //                 status = $3,
    //                 description = $4,
    //                 image = $5,
    //                 history = $6,
    //                 updated_at = $7
    //             WHERE id = $8
    //             "#,
    //             product.title,
    //             product.price,
    //             product.status,
    //             product.description,
    //             product.image,
    //             Value::Array(history),
    //             Utc::now(),
    //             existing.id
    //         )
    //             .execute(pool)
    //             .await?;
    //     }
    // }

    if !new_products.is_empty() {
        insert_products(pool, &new_products).await?;
    }

    Ok(())
}

async fn insert_products(pool: &PgPool, products: &[&Product]) -> Result<(), Box<dyn Error>> {
    let total = products.len();
    for chunk in products.chunks(BATCH_SIZE) {
        // let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
        //     "INSERT INTO products (id, p_ref, section, source, title, description, url, image, status, price, history, added_at) "
        // );
        //
        // query_builder.push_values(chunk, |mut b, product| {
        //     b.push_bind(product.id.clone())
        //         .push_bind(product.p_ref.clone())
        //         .push_bind(product.section.clone())
        //         .push_bind(product.source.clone())
        //         .push_bind(product.title.clone())
        //         .push_bind(product.description.clone())
        //         .push_bind(product.url.clone())
        //         .push_bind(product.image.clone())
        //         .push_bind(product.status.clone())
        //         .push_bind(product.price)
        //         .push_bind(Value::Array(vec![]))
        //         .push_bind(Utc::now());
        // });
        //
        // query_builder.build().execute(pool).await?;
    }

    println!("NEW: {} products inserted in {} batches", total, (total + BATCH_SIZE - 1) / BATCH_SIZE);
    Ok(())
}

async fn archive_missing_products(pool: &PgPool, ids: &HashSet<&String>) -> Result<(), sqlx::Error> {
    let db_products = sqlx::query!(
        r#"
        SELECT id FROM products
        "#
    )
        .fetch_all(pool)
        .await?;

    let mut map = MISSING_MAP.lock().await;
    let mut to_archive = Vec::new();

    for db in db_products {
        if !ids.contains(&db.id) {
            let count = map.entry(db.id.clone()).or_insert(0);
            *count += 1;

            if *count >= UNSEEN_TIMES_TO_ARCHIVE {
                to_archive.push(db.id.clone());
            }
        }
    }

    if !to_archive.is_empty() {
        archive_products(pool, &to_archive).await?;

        for id in &to_archive {
            map.remove(id);
        }
    }

    Ok(())
}

async fn archive_products(pool: &PgPool, ids: &[String]) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    for chunk in ids.chunks(BATCH_SIZE) {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"
            INSERT INTO products_archive
            (id, p_ref, section, source, title, description, url, image, status, price, history, added_at, removed_at, updated_at, created_at)
            SELECT id, p_ref, section, source, title, description, url, image, status, price, history, added_at, "#
        );

        query_builder.push_bind(now);

        query_builder.push(
            r#", updated_at, created_at
            FROM products
            WHERE id IN (
            "#
        );

        let mut separated = query_builder.separated(", ");
        for id in chunk {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        query_builder.build().execute(pool).await?;

        let mut del_builder = sqlx::QueryBuilder::new("DELETE FROM products WHERE id IN (");
        let mut del_separated = del_builder.separated(", ");
        for id in chunk {
            del_separated.push_bind(id);
        }
        del_separated.push_unseparated(")");

        del_builder.build().execute(pool).await?;
    }

    println!("REMOVED: {} products archived and deleted", ids.len());
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