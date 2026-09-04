use crate::core::product::Product;
use crate::core::tracking::error_tracker::ErrorCycleSummary;
use crate::core::tracking::scan_cache::{ScanRecord, ScanTrigger};
use crate::core::tracking::scan_report::FailedScope;
use crate::discord::config::DiscordConfig;
use crate::discord::embeds;
use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::core::tracking::scan_metrics::ScanMetrics;

static EVENT_SENDER: OnceCell<mpsc::UnboundedSender<DiscordEvent>> = OnceCell::new();
static SCAN_EVENT_SENDER: OnceCell<mpsc::UnboundedSender<DiscordEvent>> = OnceCell::new();

#[derive(Clone, Copy, Debug)]
pub enum ProductChangeKind {
    New,
    Edited,
    Removed,
    Viewed,
}

#[derive(Clone, Debug)]
pub struct AlertEvent {
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSummary {
    pub completed_at: DateTime<Utc>,
    pub duration_ms: u128,
    pub total_products: usize,
    pub added: usize,
    pub edited: usize,
    pub removed: usize,
    pub scrape_errors: usize,
    pub error_health: ErrorCycleSummary,
    pub pages: usize,
    pub failed_pages: usize,
    pub failed_scopes: Vec<FailedScope>,
    pub attempts: usize,
    pub sections_scanned: usize,
    pub sites_scanned: usize,
    pub change_sites: Vec<(String, usize, usize, usize)>,
    pub change_sections: Vec<(String, usize, usize, usize)>,
    pub top_retailers: Vec<(String, usize)>,
    pub top_sections: Vec<(String, usize)>,
    pub catalog: ScanCatalogMetrics,
    pub site_metrics: Vec<ScanSiteMetric>,
    pub section_metrics: Vec<ScanSectionMetric>,
    pub metrics: ScanMetrics,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanCatalogMetrics {
    pub in_stock: usize,
    pub out_of_stock: usize,
    pub on_arrive: usize,
    pub on_request: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSiteMetric {
    pub site: String,
    pub products: usize,
    pub pages: usize,
    pub errors: usize,
    pub duration_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSectionMetric {
    pub section: String,
    pub products: usize,
    pub sites: usize,
    pub errors: usize,
    pub duration_ms: u128,
}

#[derive(Clone, Debug)]
pub enum DiscordEvent {
    Product {
        kind: ProductChangeKind,
        product: Product,
        changes: Vec<Value>,
    },
    Alert(AlertEvent),
    Scan(ScanRecord),
    ScanStarted {
        started_at: DateTime<Utc>,
        trigger: ScanTrigger,
        sections: Vec<String>,
        retailers: Vec<String>,
    },
}

pub fn initialize(http: Arc<Http>, config: DiscordConfig) {
    let (sender, receiver) = mpsc::unbounded_channel();
    let (scan_sender, scan_receiver) = mpsc::unbounded_channel();
    
    let _ = EVENT_SENDER.set(sender);
    let _ = SCAN_EVENT_SENDER.set(scan_sender);

    for mut receiver in [receiver, scan_receiver] {
        let http = http.clone();
        let config = config.clone();
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let result = match event {
                    DiscordEvent::Product {
                        kind,
                        product,
                        changes,
                    } => {
                        let components = if matches!(kind, ProductChangeKind::Removed) {
                            embeds::removed_product_actions(&product.url)
                        } else {
                            embeds::product_actions(&product)
                        };
                        let message = CreateMessage::new()
                            .embed(embeds::product(&product, kind, &changes))
                            .components(components);
                        config
                            .product_channel(kind)
                            .send_message(&http, message)
                            .await
                            .map(|_| ())
                    }
                    DiscordEvent::Scan(record) => config
                        .scan_channel
                        .send_message(
                            &http,
                            CreateMessage::new()
                                .embed(embeds::scan_overview(&record))
                                .components(embeds::scan_actions(&record.id)),
                        )
                        .await
                        .map(|_| ()),
                    DiscordEvent::ScanStarted {
                        started_at,
                        trigger,
                        sections,
                        retailers,
                    } => config
                        .scan_channel
                        .send_message(
                            &http,
                            CreateMessage::new().embed(embeds::scan_started(
                                started_at,
                                &trigger,
                                &sections,
                                &retailers,
                            )),
                        )
                        .await
                        .map(|_| ()),
                    DiscordEvent::Alert(alert) => config
                        .alert_channel
                        .send_message(&http, CreateMessage::new().embed(embeds::alert(&alert)))
                        .await
                        .map(|_| ()),
                };

                if let Err(error) = result {
                    eprintln!("Failed to send Discord event: {error}");
                }
            }
        });
    }
}

pub fn emit(event: DiscordEvent) {
    let sender = if matches!(event, DiscordEvent::Scan(_) | DiscordEvent::ScanStarted { .. }) {
        SCAN_EVENT_SENDER.get()
    } else {
        EVENT_SENDER.get()
    };
    if let Some(sender) = sender {
        let _ = sender.send(event);
    }
}

pub fn alert(
    level: impl Into<String>,
    target: impl Into<String>,
    message: impl Into<String>,
    fields: Vec<(String, String)>,
) {
    emit(DiscordEvent::Alert(AlertEvent {
        level: level.into(),
        target: target.into(),
        message: message.into(),
        fields,
    }));
}
