use axum::http::HeaderMap;
use once_cell::sync::OnceCell;
use playwright_rs::{Browser, BrowserContextOptions, LaunchOptions, Playwright};
use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
const TIMEOUT: Duration = Duration::from_secs(30);

static WEB_CLIENT: OnceCell<WebClient> = OnceCell::new();

pub enum WebClientType {
    HttpClient,
    Browser
}

pub struct WebClient {
    pub http_client: Client,
    pub playwright: Playwright,
    pub browser: Browser,
}

impl WebClient {
    pub async fn init() {
        let http_client = ClientBuilder::new()
            .user_agent(USER_AGENT)
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".parse().unwrap());
                headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
                headers
            })
            .timeout(TIMEOUT)
            .build()
            .expect("Failed to build HTTP client");

        let playwright = Playwright::launch().await.expect("Failed to launch Playwright");
        let browser = playwright.chromium()
            .launch_with_options(LaunchOptions {
                headless: Some(true),
                args: Some(vec![
                    "--disable-gpu".to_string(),
                    "--disable-dev-shm-usage".to_string(),
                    "--disable-plugins".to_string(),
                    "--disable-image-loading".to_string(),
                ]),
                ..Default::default()
            }).await.expect("Failed to launch chromium");

        let _ = WEB_CLIENT.set(WebClient {
            http_client,
            playwright,
            browser,
        });
    }

    pub async fn fetch(url: &str, web_client_type: &WebClientType) -> Result<String, Box<dyn Error>> {
        println!("Sending http request to `{url}`");
        let web_client = WEB_CLIENT.get().unwrap();
        match web_client_type {
            WebClientType::HttpClient => {
                let response = web_client.http_client.get(url).send().await?;
                let body = response.text().await?;
                Ok(body)
            },
            WebClientType::Browser => {
                let context = web_client.browser.new_context_with_options(BrowserContextOptions {
                    user_agent: Some(USER_AGENT.to_string()),
                    extra_http_headers: Some(HashMap::from([
                        ("Accept-Language".to_string(), "en-US,en;q=0.5".to_string()),
                    ])),
                    ..Default::default()
                }).await.expect("Failed to build browser context");

                let page = context.new_page().await?;
                page.goto(url, None).await?;
                page.wait_for_load_state(None).await?;
                sleep(Duration::from_secs(1)).await;

                let body = page.content().await?;
                page.close().await?;
                context.close().await?;

                if body.contains("Sorry, you have been blocked") {
                    eprintln!("Blocked from {url}");
                }

                Ok(body)
            }
        }
    }

    pub async fn cleanup() -> Result<(), Box<dyn Error>> {
        if let Some(client) = WEB_CLIENT.get() {
            client.browser.close().await?;
            client.playwright.shutdown().await?
        }
        Ok(())
    }
}