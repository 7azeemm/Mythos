use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use once_cell::sync::Lazy;
use crate::web_scraper::specs::pc_specs::{CPUSpecs};

pub static CPU_DATASET: Lazy<HashMap<String, CPUSpecs>> = Lazy::new(|| {
    load_cpu_dataset().expect("Failed to load CPU dataset")
});

// pub static GPU_DATASET: Lazy<HashMap<String, GPUSpecs>> = Lazy::new(|| {
//     load_gpu_dataset().expect("Failed to load GPU dataset")
// });

pub fn load_datasets() {
    Lazy::force(&CPU_DATASET);
    // Lazy::force(&GPU_DATASET);
    // println!("Datasets loaded: {} CPUs, {} GPUs", CPU_DATASET.len(), GPU_DATASET.len());
}

fn load_cpu_dataset() -> Result<HashMap<String, CPUSpecs>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("datasets/cpu-dataset.csv")?;
    let mut map = HashMap::new();

    for cpu in rdr.deserialize::<CPUSpecs>() {
        let cpu = cpu?;
        map.insert(cpu.name.clone(), cpu);
    }

    Ok(map)
}

// fn load_gpu_dataset() -> Result<HashMap<String, GPUSpecs>, Box<dyn Error>> {
//     let mut rdr = Reader::from_path("datasets/gpu-dataset.csv")?;
//     let mut map = HashMap::new();
//
//     for gpu in rdr.deserialize::<GPUSpecs>() {
//         let gpu = gpu?;
//         map.insert(gpu.name.clone(), gpu);
//     }
//
//     Ok(map)
// }