use crate::web_scraper::specs::component_specs::ComponentSpecs;
use crate::web_scraper::specs::ProductSpecs;
use std::error::Error;

pub fn parse_component(description: &str) -> Result<ProductSpecs, Box<dyn Error>> {
    let mut parts = description.split("- ").collect::<Vec<&str>>().into_iter();

    let mut functions: Vec<(Vec<&str>, Box<dyn FnMut(&str)>)> = vec![
    ];

    while let Some(part) = parts.next() {
        let part_lower = part.to_lowercase();

        let func = functions.iter_mut().find_map(|(keys, func)| {
            let mut matches = false;
            let mut excluded = false;

            for key in keys {
                if let Some(exclude_word) = key.strip_prefix('!') {
                    if part_lower.contains(exclude_word) {
                        excluded = true;
                        break;
                    }
                } else if part_lower.contains(*key) {
                    matches = true;
                }
            }

            if matches && !excluded { Some(func) } else { None }
        });

        let Some(func) = func else {
            // print(part);
            continue
        };

        (*func)(part);
    }
    
    Ok(ProductSpecs::Component(ComponentSpecs {
        
    }))
}