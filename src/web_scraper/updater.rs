use crate::utils::database::get_db_pool;
use crate::web_scraper::errors::{CycleReport, UpdateError, UpdateErrorKind};
use crate::web_scraper::product::Product;
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use crate::web_scraper::sections::Section;

const BATCH_SIZE: usize = 250;

pub struct ProductUpdater;

impl ProductUpdater {
    pub async fn archive_missing_products(
        report: &mut CycleReport,
        products: &Vec<Product>,
        sections: &Option<Vec<Section>>,
        sites: &Option<Vec<&'static str>>
    ) {
        let pool = get_db_pool();

        let mut to_archive: Vec<Product> = match sqlx::query_as::<_, Product>(r#"
            SELECT *
            FROM products
            WHERE url NOT IN (SELECT UNNEST($1::TEXT[]))
                  AND ($2::TEXT[] IS NULL OR section = ANY($2::TEXT[]))
                  AND ($3::TEXT[] IS NULL OR site = ANY($3::TEXT[]))
        "#)
            .bind(products.iter().map(|p| p.url.as_str()).collect::<Vec<_>>())
            .bind(sections)
            .bind(sites)
            .fetch_all(pool)
            .await
        {
            Ok(p) => p,
            Err(err) => {
                report.update.errors.push(UpdateError {
                    error: UpdateErrorKind::FetchMissingProducts,
                    message: err.to_string(),
                    timestamp: Utc::now()
                });
                return
            }
        };

        if to_archive.is_empty() {
            return
        }

        let now = Utc::now();
        for product in to_archive.iter_mut() {
            product.removed_at = Some(now.clone());
        }

        for chunk in to_archive.chunks(BATCH_SIZE) {
            // let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"
            //     INSERT INTO archive
            //     (url, name, title, site, section, description, image, status, price,
            //      old_price, specs, history, added_at, removed_at)
            // "#);
            //
            // bind_product(&mut query_builder, chunk, false);
            //
            // if let Err(err) = query_builder.build().execute(pool).await {
            //     report.update.errors.push(UpdateError {
            //         error: UpdateErrorKind::InsertToArchive,
            //         message: err.to_string(),
            //         timestamp: Utc::now()
            //     });
            //     return
            // }

            if let Err(err) = sqlx::query("DELETE FROM products WHERE url = ANY($1)")
                .bind::<Vec<&String>>(chunk.iter().map(|p| &p.url).collect())
                .execute(pool)
                .await
            {
                report.update.errors.push(UpdateError {
                    error: UpdateErrorKind::DeleteProducts,
                    message: err.to_string(),
                    timestamp: Utc::now()
                });
                return
            }
        }

        report.update.removed += to_archive.len();
        report.removed_items.extend(to_archive);
    }

    pub async fn sync(report: &mut CycleReport, products: Vec<Product>) -> Vec<Product> {
        let pool = get_db_pool();
        let now = Utc::now();
        let urls: Vec<&str> = products.iter().map(|p| p.url.as_str()).collect();
        let mut new_products = Vec::new();

        let db_products: Vec<Product> = match sqlx::query_as::<_, Product>(r#"
            SELECT *
            FROM products
            WHERE url = ANY($1)
        "#)
            .bind(urls)
            .fetch_all(pool)
            .await
        {
            Ok(p) => p,
            Err(err) => {
                report.update.errors.push(UpdateError {
                    error: UpdateErrorKind::SelectProducts,
                    message: err.to_string(),
                    timestamp: Utc::now()
                });
                return new_products
            }
        };

        let mut map = HashMap::new();
        for p in db_products {
            map.insert(p.url.clone(), p);
        }

        for mut product in products {
            let Some(db_product) = map.get(&product.url) else {
                // New Product
                new_products.push(product);
                continue
            };

            let title_changed = product.title != db_product.title;
            let desc_changed = product.description != db_product.description;
            let image_changed = product.image != db_product.image;
            let status_changed = product.status != db_product.status;
            let price_changed = product.price != db_product.price;
            let old_price_changed = product.old_price != db_product.old_price;

            // Changed Product
            if title_changed | desc_changed | image_changed | status_changed | price_changed | old_price_changed {
                fn push_change<T: Serialize>(history: &mut Vec<Value>, field: &str, old: T, new: T) {
                    history.push(json!({
                        "field": field,
                        "old_value": old,
                        "new_value": new,
                        "timestamp": Utc::now()
                    }));
                }

                let mut changes = Vec::new();

                if title_changed { push_change(&mut changes, "title", &product.title, &db_product.title); }
                if desc_changed { push_change(&mut changes, "description", &product.description, &db_product.description); }
                if image_changed { push_change(&mut changes, "image", &product.image, &db_product.image); }
                if status_changed { push_change(&mut changes, "status", &product.status, &db_product.status); }
                if price_changed { push_change(&mut changes, "price", &product.price, &db_product.price); }
                if old_price_changed { push_change(&mut changes, "old_price", &product.old_price, &db_product.old_price); }

                report.update.edited += 1;
                report.edited_items.push((product.clone(), changes.clone()));

                let mut history = db_product.history.as_array().cloned().unwrap_or_default();
                history.extend(changes);

                product.history = Value::Array(history);
                product.updated_at = Some(now.clone());

                if let Err(error) = Self::update_product(product).await {
                    report.update.errors.push(error);
                }
            }
        }

        new_products
    }

    pub async fn insert_products(report: &mut CycleReport, products: Vec<Product>) {
        for chunk in products.chunks(BATCH_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(r#"
                INSERT INTO products
                (url, name, title, site, section, description, image, status, price,
                 old_price, specs, history, added_at, updated_at)
            "#);

            bind_product(&mut query_builder, chunk, true);

            if let Err(err) = query_builder.build().execute(get_db_pool()).await {
                report.update.errors.push(UpdateError {
                    error: UpdateErrorKind::InsertProducts,
                    message: err.to_string(),
                    timestamp: Utc::now()
                });
            }
        }

        report.update.added += products.len();
        report.added_items.extend(products);
    }

    async fn update_product(product: Product) -> Result<(), (UpdateError)> {
        match sqlx::query!(r#"
            UPDATE products
            SET title = $1,
                description = $2,
                image = $3,
                status = $4,
                price = $5,
                old_price = $6,
                history = $7,
                updated_at = $8
            WHERE url = $9
            "#,
            product.title,
            product.description,
            product.image,
            &product.status.to_string(),
            product.price,
            product.old_price,
            product.history,
            product.updated_at,
            product.url
        )
            .execute(get_db_pool())
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(UpdateError {
                error: UpdateErrorKind::UpdateProduct,
                message: err.to_string(),
                timestamp: Utc::now()
            })
        }
    }
}

fn bind_product(builder: &mut QueryBuilder<Postgres>, chunk: &[Product], update: bool) {
    builder.push_values(chunk, |mut b, product| {
        b.push_bind(&product.url)
            .push_bind(&product.name)
            .push_bind(&product.title)
            .push_bind(&product.site)
            .push_bind(&product.section)
            .push_bind(&product.description)
            .push_bind(&product.image)
            .push_bind(&product.status)
            .push_bind(&product.price)
            .push_bind(&product.old_price)
            .push_bind(&product.specs)
            .push_bind(&product.history)
            .push_bind(&product.added_at)
            .push_bind(if update { &product.updated_at } else { &product.removed_at });
    });
}