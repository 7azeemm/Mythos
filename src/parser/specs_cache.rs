use crate::parser::parser::parse_specs;
use crate::parser::specs::PCSpecs;
use crate::utils::database::get_db_pool;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub static SPECS_CACHE: Lazy<RwLock<SpecsCacheData>> = Lazy::new(|| RwLock::new(SpecsCacheData::default()));

#[derive(Debug, Clone, Default)]
pub struct SpecsCacheData {
    pub specs: HashMap<String, PCSpecs>,
    pub cpus: HashMap<String, Vec<String>>,
    pub gpus: HashMap<String, Vec<String>>,
    pub ram_types: HashMap<String, Vec<String>>,
    pub storage_types: HashMap<String, Vec<String>>,
}

impl SpecsCacheData {
    fn add_product_specs(&mut self, product_id: String, description: &str) {
        let specs = match parse_specs(description) {
            Ok(specs) => specs,
            Err(err) => {
                eprintln!("Failed to parse product {product_id}: {err}");
                return
            }
        };
        
        self.cpus
            .entry(specs.cpu.name.clone())
            .or_insert_with(Vec::new)
            .push(product_id.clone());

        self.gpus
            .entry(specs.gpu.name.clone())
            .or_insert_with(Vec::new)
            .push(product_id.clone());

        self.ram_types
            .entry(format!("{:?}", specs.memory.ram_type))
            .or_insert_with(Vec::new)
            .push(product_id.clone());

        self.storage_types
            .entry(format!("{:?}", specs.storage.storage_type))
            .or_insert_with(Vec::new)
            .push(product_id.clone());

        self.specs.insert(product_id, specs);
    }

    pub fn get_cpu_count(&self, cpu_name: &str) -> i32 {
        self.cpus
            .get(cpu_name)
            .map(|ids| ids.len() as i32)
            .unwrap_or(0)
    }

    pub fn get_gpu_count(&self, gpu_name: &str) -> i32 {
        self.gpus
            .get(gpu_name)
            .map(|ids| ids.len() as i32)
            .unwrap_or(0)
    }

    pub fn get_ram_type_count(&self, ram_type: &str) -> i32 {
        self.ram_types
            .get(ram_type)
            .map(|ids| ids.len() as i32)
            .unwrap_or(0)
    }

    pub fn get_storage_type_count(&self, storage_type: &str) -> i32 {
        self.storage_types
            .get(storage_type)
            .map(|ids| ids.len() as i32)
            .unwrap_or(0)
    }

    pub fn filter_products_by_cpu(&self, cpu_name: &str) -> Vec<String> {
        self.cpus
            .get(cpu_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn filter_products_by_gpu(&self, gpu_name: &str) -> Vec<String> {
        self.gpus
            .get(gpu_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn filter_products_by_ram_type(&self, ram_type: &str) -> Vec<String> {
        self.ram_types
            .get(ram_type)
            .cloned()
            .unwrap_or_default()
    }

    pub fn filter_products_by_storage_type(&self, storage_type: &str) -> Vec<String> {
        self.storage_types
            .get(storage_type)
            .cloned()
            .unwrap_or_default()
    }
}

pub async fn initialize_cache() {
    let mut cache = SPECS_CACHE.write().await;
    *cache = SpecsCacheData::default();

    let products: Vec<(String, String)> = match sqlx::query_as(
        "SELECT id, description FROM products"
    )
        .fetch_all(get_db_pool())
        .await
    {
        Ok(products) => products,
        Err(err) => {
            eprintln!("Failed to retrieve products from database for cache: {err}");
            return;
        }
    };

    for (product_id, description) in products {
        cache.add_product_specs(product_id, &description);
    }

    println!("Specs cache initialized with {} products", cache.specs.len());
}