**Phase 1 — Foundation**

- [ ]  Decide fate of `Specs` struct
- [ ]  Create mobile/laptop GPU dataset
- [ ]  Fill out datasets
- [ ]  Fix grouping mode

**Phase 2 — Discord Bot**

- [ ]  Pick Discord library (serenity vs. twilight)
- [ ]  Slash commands + admin-only gate
- [ ]  Wire `CycleReport`/`KnownEvents` to actual Discord messages
- [ ]  Decide message batching/format
- [ ]  Review queue UI/commands for `NotInDataset` items
- [ ]  Fuzzy-match top-5 suggestions (pick a crate: `strsim`/`fuzzy-matcher`)
- [ ]  Admin command: edit product fields
- [ ]  Admin command: fix wrong dataset match
- [ ]  Admin command: add/edit dataset entries
- [ ]  Admin command: edit section config
- [ ]  Atomic writes for all Discord-driven JSON edits

**Phase 3 — Site Robustness**

- [ ]  Image validity checker (HTTP check/Page fetch)
- [ ]  New-category detector per site

**Phase 4 — AI Dataset Maintenance**

- [ ]  Detect genuinely-new chipset/model triggers from review queue
- [ ]  Web search + spec extraction for the unknown item
- [ ]  Propose dataset JSON diff to Discord
- [ ]  Human-approval gate before any write
- [ ]  Dedup repeated proposals via fingerprinting

**Phase 5 — Component Storage**

- [ ]  Design Component schema (spec fields + price history + aggregate stats)
- [ ]  Key by existing `filter_ids`
- [ ]  Decide storage format (JSON vs. lightweight embedded DB) — do this before coding
- [ ]  Wire Component upsert into the existing parse-cycle update flow

**Phase 6 — Price History & Analytics**

- [ ]  Per-component price charts
- [ ]  Combined-factor aggregation layer
- [ ]  Pick first 3-4 combined-factor charts to ship
- [ ]  Decide precompute vs. compute-on-read for aggregates

**Phase 7 — Monetization**

- [ ]  User accounts (auth strategy decision, password hashing)
- [ ]  Tiered alert system (schema, matching engine, delivery)
- [ ]  Benchmark data source decision for scoring (UL / PassMark / curated tier list)
- [ ]  Map benchmark scores onto existing CPU/GPU dataset entries
- [ ]  On-demand refresh (single-product v1, rate-limited)
- [ ]  Additional paid features: full price-history access, saved watchlists, restock alerts, delivery-channel choice, CSV export, API access, priority site requests

**Phase 8 — Backlog (not now)**

- [ ]  PC Builder section
- [ ]  Marketplace
