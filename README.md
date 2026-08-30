# Mythos

**A platform in development for discovering tech products, understanding prices, and planning purchases in Tunisia.**

Mythos brings together product data from Tunisian retailers as the foundation for a broader shopping platform: price-history graphs, product trackers and alerts, a marketplace, PC building tools, and more.

This repository currently contains the **Rust data collection and API backend**. The customer-facing website and the broader platform features are still on the roadmap.

## What is here today

- Retailer adapters for stores including Mytek, Tunisianet, Spacenet, MegaPC, and others.
- HTTP and browser-based collection infrastructure using Reqwest and Playwright.
- Category-specific parsers and reference datasets for normalizing product information.
- Product records with retailer links, prices, availability, component identifiers, and change-history fields.
- An Axum API with search, price and stock filters, retailer selection, sorting, pagination, and experimental product grouping.
- JSON product exports, page and description caches, and scraping-cycle reports.

> [!IMPORTANT]
> The current development path in `ProductManager::fetch_sites` returns cached pages before reaching the live scraper. A fresh checkout without `pages_cache.json` can therefore serve an empty catalogue. The product-file loading code in `ProductStorage::load` is also commented out. This is a development backend, not a ready-to-deploy tracking service.

## Platform roadmap

| Area | Direction |
| --- | --- |
| Product discovery | A searchable website with richer product details and comparisons |
| Graphs and analytics | Price-history charts and availability trends |
| Trackers | Watchlists, price-drop alerts, restock alerts, and advanced queries |
| PC builder | Tools for choosing components and planning a build |
| Marketplace | A marketplace alongside retailer product discovery |
| Product insights | Product reviews, scoring, and benchmark-informed comparisons |

These are planned capabilities, not a list of features already available in this repository. See [PROJECT_ROADMAP.md](PROJECT_ROADMAP.md) for the working development plan; priorities may change.

## How the backend is organized

| Location | Responsibility |
| --- | --- |
| [src/web_scraper/sites](src/web_scraper/sites) | Retailer-specific collection logic |
| [src/web_scraper/parsers](src/web_scraper/parsers) | Category parsing and normalization |
| [config/sections.json](config/sections.json) | Category rules, filters, and grouping fields |
| [config/datasets](config/datasets) | Reference data used during parsing |
| [src/web_scraper/manager.rs](src/web_scraper/manager.rs) | Collection, parsing, caching, and update cycles |
| [src/storage.rs](src/storage.rs) | In-memory product storage, updates, and JSON exports |
| [src/api](src/api) | HTTP routes, queries, filters, and responses |

The data flow is retailer adapters or cached pages → normalization and change detection → product storage → API responses. Products are exported to `data/<Section>.json`; cycle reports go to `reports/`.

## Local development

### Requirements

- A recent stable Rust toolchain supporting the Rust 2024 edition.
- Playwright's Chromium browser and its system dependencies, compatible with the `playwright-rs` version in [Cargo.toml](Cargo.toml). The application initializes Chromium even when using cached data.
- Local development data if you want to exercise the current cached-page path.

```bash
git clone https://github.com/7azeemm/Mythos.git
cd Mythos
cargo build --locked
cargo run --locked
```

The Cargo package is still named `PriceTracker`; the project is now called **Mythos**. Run commands from the repository root so relative configuration and data paths resolve correctly.

The server currently binds to `0.0.0.0:3000`. Use it in a trusted local environment: CORS is permissive, and the current router has no authentication layer. If Chromium is missing, install the browser revision required by the pinned Playwright integration before starting again.

### Try the API

```bash
curl http://localhost:3000/info
curl 'http://localhost:3000/GPU/products?page=1&sort=price_desc&require_section_info=true'
curl 'http://localhost:3000/Laptop/products?search=Lenovo&stock=InStock'
```

Use the section identifiers returned by `/info`; they are case-sensitive. Product pages contain up to 60 products, or 60 groups when grouping is enabled.

| Query | Purpose |
| --- | --- |
| `search` | Match product titles or descriptions |
| `min_price`, `max_price` | Restrict the price range |
| `site`, `stock` | Filter retailer names and stock states |
| `page` | Select a page, starting at 1 |
| `sort=price_desc` | Sort by descending price; default is ascending |
| `require_section_info=true` | Include available filter and category metadata |
| `grouping_mode=true` | Return experimental product groups |
| `filters` | JSON object mapping filter keys to lists of accepted identifiers |

## Development notes

Parser coverage, grouping accuracy, data restoration, and durable history storage are still being refined. A history field in a product record does not yet mean that a complete charting or alert service exists.

When enabling live collection, review retailer access policies and use appropriate request limits. Treat collected prices and availability as observations, and verify them with the retailer before purchasing. Keep credentials and private operational data out of Git.

## Contributing

Useful contributions include reproducible parser failures, sanitized sample pages, normalization improvements, and focused API fixes. Discuss larger changes before implementing them so they fit the platform roadmap.
