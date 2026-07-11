use std::error::Error;
use axum::{middleware, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::api::endpoints;
use crate::api::middleware::logging_middleware;

pub async fn run(port: u16) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/info", get(endpoints::info::get_info))
        .route("/{section}/products", get(endpoints::products::get_products))

        .layer(CorsLayer::permissive())// For Debugging ONLY
        .layer(middleware::from_fn(logging_middleware));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;
    
    Ok(())
}