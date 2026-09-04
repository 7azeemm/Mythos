use crate::core::tracking::scan_report::PageReport;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ScanMetrics {
    pub duplicates_removed: usize,
    pub moved_sections: usize,
    pub pages: PageMetrics,
    pub next_scheduled_at: Option<DateTime<Utc>>,
}

impl ScanMetrics {
    pub fn record_pages(&mut self, pages: &[PageReport]) {
        self.pages = PageMetrics::default();
        for page in pages {
            self.pages.add(&page.metrics);
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PageMetrics {
    pub description_cache_hits: usize,
    pub description_requests: usize,
    pub html_bytes: u64,
}

impl PageMetrics {
    pub fn add(&mut self, other: &Self) {
        self.description_cache_hits += other.description_cache_hits;
        self.description_requests += other.description_requests;
        self.html_bytes += other.html_bytes;
    }
}
