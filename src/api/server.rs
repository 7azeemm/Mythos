use std::error::Error;
use axum::{middleware, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use crate::api::handlers;
use crate::api::middleware::logging_middleware;

pub async fn run(port: u16) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/products", get(handlers::products::list))
        .route("/api/products/{id}", get(handlers::products::get_by_id))
        // .route("/api/search", get(handlers::products::search))
        .layer(middleware::from_fn(logging_middleware));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;
    
    Ok(())
}

pub async fn health_check() -> &'static str {
    "OK"
}