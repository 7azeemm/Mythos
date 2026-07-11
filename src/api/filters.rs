use crate::web_scraper::dataset::{Dataset, FilterNode};
use crate::web_scraper::product::Product;
use crate::web_scraper::sections::Section;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

static STRUCTURED_FILTERS: &[(&'static str, Section)] = &[
    ("cpu", Section::CPU),
    ("gpu", Section::GPU),
    ("chipset", Section::GPU),
    ("model", Section::GamingLaptop)
];

#[derive(Debug, Serialize)]
pub struct FilterGroup {
    pub key: String,
    pub label: String,
    pub options: Vec<FilterValue>,
}

#[derive(Debug, Serialize)]
pub struct FilterValue {
    pub id: String,
    pub label: String,
    pub count: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<FilterValue>,
}

fn count_ids<'a>(ids: impl Iterator<Item = &'a str>) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for id in ids {
        *counts.entry(id.to_string()).or_insert(0) += 1;
    }
    counts
}

fn apply_counts(node: &FilterNode, counts: &HashMap<String, usize>) -> FilterValue {
    if node.children.is_empty() {
        FilterValue {
            id: node.id.clone(),
            label: node.label.clone(),
            count: counts.get(&node.id).copied().unwrap_or(0),
            children: vec![],
        }
    } else {
        let children: Vec<FilterValue> = node.children.iter().map(|c| apply_counts(c, counts)).collect();
        let count = children.iter().map(|c| c.count).sum();
        FilterValue { id: node.id.clone(), label: node.label.clone(), count, children }
    }
}

fn build_others_node(counts: &HashMap<String, usize>, known: &HashSet<&str>) -> Option<FilterValue> {
    let mut children: Vec<FilterValue> = counts.iter()
        .filter(|(id, _)| !known.contains(id.as_str()))
        .map(|(id, count)| {
            if !id.starts_with("Others/") {
                eprintln!("filter id `{id}` missing from dataset tree and not namespaced as Others");
            }
            let label = id.rsplit('/').next().unwrap_or(id).to_string();
            FilterValue { id: id.clone(), label, count: *count, children: vec![] }
        })
        .collect();

    if children.is_empty() { return None; }
    let total = children.iter().map(|c| c.count).sum();
    Some(FilterValue { id: "Others".into(), label: "Others".into(), count: total, children })
}

fn merge_others(options: &mut Vec<FilterValue>, others: FilterValue) {
    if let Some(existing) = options.iter_mut().find(|o| o.id == "Others") {
        existing.count += others.count;
        existing.children.extend(others.children);
    } else {
        options.push(others);
    }
}

fn extract_numeric_value(label: &str) -> Option<f64> {
    // Extract leading digits and dots (e.g., "1.5", "0.5", "512")
    let num_str: String = label.chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    if num_str.is_empty() || num_str == "." {
        return None;
    }
    let mut number: f64 = num_str.parse().ok()?;
    if label.contains("TB") {
        number *= 1024.0; // convert TB to GB-equivalent
    }
    Some(number)
}

fn sort_options(options: &mut [FilterValue]) {
    options.sort_by(|a, b| {
        // 1) "Others" always last
        let others_cmp = a.id.ends_with("Others").cmp(&b.id.ends_with("Others"));
        if others_cmp != std::cmp::Ordering::Equal {
            return others_cmp;
        }

        // 2) If both labels start with a number, sort numerically
        let a_num = extract_numeric_value(&a.label);
        let b_num = extract_numeric_value(&b.label);
        if let (Some(an), Some(bn)) = (a_num, b_num) {
            return an.total_cmp(&bn);
        }

        // 3) Fallback: alphabetical by label, then by count descending
        a.label.cmp(&b.label).then_with(|| b.count.cmp(&a.count))
    });

    for opt in options.iter_mut() {
        sort_options(&mut opt.children);
    }
}

/// Drops zero-count nodes recursively. A group node's count is always the
/// sum of its (already-pruned) children, so if all children get removed the
/// group naturally ends up at count 0 and gets removed too.
fn prune_zero_counts(options: Vec<FilterValue>) -> Vec<FilterValue> {
    options.into_iter()
        .filter_map(|mut v| {
            if !v.children.is_empty() {
                v.children = prune_zero_counts(v.children);
            }
            (v.count > 0).then_some(v)
        })
        .collect()
}

fn label_from_key(key: &str, structured: bool) -> String {
    if structured {
        return key.to_uppercase();
    }
    key.split('_')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_structured_filter_group(key: &str, dataset: &Dataset, products: &[&Product]) -> FilterGroup {
    let counts = count_ids(products.iter().filter_map(|p| p.filter_ids.get(key).map(String::as_str)));
    let known: HashSet<&str> = dataset.nodes.iter().map(|n| n.id.as_str()).collect();

    let mut options: Vec<FilterValue> = dataset.tree.iter().map(|n| apply_counts(n, &counts)).collect();

    if let Some(others) = build_others_node(&counts, &known) {
        merge_others(&mut options, others);
    }

    let missing = products.iter().filter(|p| !p.filter_ids.contains_key(key)).count();
    if missing > 0 {
        merge_others(&mut options, FilterValue {
            id: "Others".into(),
            label: "Others".into(),
            count: missing,
            children: vec![FilterValue {
                id: "Others/Others".into(),
                label: "Others".into(),
                count: missing,
                children: vec![],
            }],
        });
    }

    options = prune_zero_counts(options);
    sort_options(&mut options);

    FilterGroup { key: key.to_string(), label: label_from_key(key, true), options }
}

fn build_simple_filter_group(key: &str, products: &[&Product]) -> FilterGroup {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for p in products {
        if let Some(v) = p.filter_ids.get(key) {
            *counts.entry(v.clone()).or_insert(0) += 1;
        }
    }

    let mut options: Vec<FilterValue> = counts.into_iter()
        .map(|(value, count)| FilterValue { id: value.clone(), label: value, count, children: vec![] })
        .collect();

    let missing = products.iter().filter(|p| !p.filter_ids.contains_key(key)).count();
    if missing > 0 {
        options.push(FilterValue { id: "Others".into(), label: "Others".into(), count: missing, children: vec![] });
    }

    sort_options(&mut options);

    FilterGroup { key: key.to_string(), label: label_from_key(key, false), options }
}

/// Products that satisfy every active selection EXCEPT exclude_key's own.
fn scoped_products<'a>(
    products: &[&'a Product],
    selections: &HashMap<String, Vec<String>>,
    exclude_key: &str,
) -> Vec<&'a Product> {
    products.iter()
        .filter(|p| {
            selections.iter().all(|(key, ids)| {
                key == exclude_key || ids.is_empty() || product_matches_key(p, key, ids)
            })
        })
        .copied()
        .collect()
}

fn matches_selection(value: &str, selected_ids: &[String]) -> bool {
    selected_ids.iter().any(|sel| {
        value == sel || (value.len() > sel.len() && value.starts_with(sel.as_str()) && value.as_bytes()[sel.len()] == b'/')
    })
}

pub fn product_matches_key(product: &Product, key: &str, selected_ids: &[String]) -> bool {
    match product.filter_ids.get(key) {
        Some(value) => matches_selection(value, selected_ids),
        None => selected_ids.iter().any(|s| s == "Others" || s == "Others/Others"),
    }
}

pub fn build_all_filters(products: &[&Product], selections: &HashMap<String, Vec<String>>) -> Vec<FilterGroup> {
    let keys = products.first().map(|p| p.section.config().filters.clone()).unwrap_or_default();
    let mut groups = Vec::with_capacity(keys.len());

    for key in keys {
        let scoped = scoped_products(products, selections, &key);

        let group = match STRUCTURED_FILTERS.iter().find(|(k, _)| *k == key) {
            Some((_, section)) => build_structured_filter_group(&key, section.parser().dataset(), &scoped),
            None => build_simple_filter_group(&key, &scoped)
        };

        groups.push(group);
    }
    groups
}