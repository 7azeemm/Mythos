use std::time::Instant;
use openrouter_rs::{
    OpenRouterClient,
    api::chat::{ChatCompletionRequest, Message},
    types::{Choice, ResponseFormat, Role},
};
use openrouter_rs::types::ProviderPreferences;
use serde::Deserialize;
use crate::utils::file_loader::FileLoader;

#[derive(Deserialize)]
pub struct TempData {
    pub model: String,
    pub provider: String,
    pub products: Vec<ProductData>
}

#[derive(Deserialize)]
pub struct ProductData {
    pub title: String,
    pub description: String
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set");
    let client = OpenRouterClient::builder()
        .api_key(api_key)
        .x_title("openrouter-rs")
        .build()?;

    let format = ResponseFormat::json_schema(
        "product_specs",
        true,
        serde_json::json!({
          "type": "object",
          "properties": {
            "cpu": {
              "type": ["string", "null"],
              "description": "CPU model name"
            },
            "gpu": {
              "type": ["string", "null"],
              "description": "GPU model with VRAM if available"
            },
            "memory": {
              "type": ["string", "null"],
              "description": "RAM with type and speed if available"
            },
            "storage": {
              "type": ["string", "null"],
              "description": "Storage size and type if available"
            }
          },
          "required": ["memory", "storage", "gpu", "cpu"]
        }),
    );

    let system_prompt = Message::new(Role::System, r#"
You are a PC hardware specification extractor. Analyze product information and extract hardware specs.

INSTRUCTIONS:
- Respond ONLY in JSON format matching the schema
- Respond in English
- Use null or empty string for unavailable fields
- Do not add fields outside the schema
- Never include parentheses, extra details, or memory types (GDDR6, GDDR5, etc.)
- Always use English units: GB or TB (never Go, Go, or other variations)

EXTRACTION RULES:

CPU:
- Format: Exact model name as found (no extra details)
- Example: "Intel Core i5-13420H"
- If not found, leave empty

GPU:
- Format: Model name [VRAM in GB] - ONLY include VRAM amount if available without memory type
- Example: "RTX 4060 Ti", "Radeon RX 9600 XT 12GB"
- Do NOT include memory type (no GDDR6, GDDR5, etc.)
- Do NOT use parentheses
- If not found, leave empty

Memory:
- Format: "[size]GB [type]"
- Include DDR type (DDR3/DDR4/DDR5) if found
- Examples: "8GB DDR4", "16GB DDR5", "16GB"
- If not found, leave empty

Storage:
- Format: "[size]GB/TB [type]"
- Include storage type (SSD/NVMe/HDD) if found
- Examples: "512GB SSD", "1TB NVMe", "2TB"
- If not found, leave empty
"#);

    let data = FileLoader::load_from_file::<TempData>("data.json").await?;

    let mut prefs = ProviderPreferences::default();
    prefs.order = Some(vec![data.provider.clone()]);

    for product in &data.products {
        let prompt = Message::new(
            Role::User,
            format!("Product Title: {}\nProduct Description: {}", product.title, product.description)
        );

        let chat_request = ChatCompletionRequest::builder()
            .model(&data.model)
            .provider(prefs.clone())
            .messages(vec![system_prompt.clone(), prompt])
            .response_format(format.clone())
            .build()?;

        let start_time = Instant::now();
        let chat_response = client.chat().create(&chat_request).await?;

        for choice in &chat_response.choices {
            if let Choice::NonStreaming(non_streaming_choice) = choice {
                if let Some(content) = &non_streaming_choice.message.content {
                    println!("Response: {content}");
                }
            }
        }

        if let Some(usage) = &chat_response.usage {
            println!("{:#?}", usage);
        }

        println!("Finished in {:.2?}", start_time.elapsed());
    }

    Ok(())
}