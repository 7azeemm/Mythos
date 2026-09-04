use crate::api::endpoints;
use crate::api::middleware::logging_middleware;
use axum::routing::get;
use axum::{Router, middleware};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub async fn run(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/info", get(endpoints::info::get_info))
        .route(
            "/{section}/products",
            get(endpoints::products::get_products),
        )
        .layer(CorsLayer::permissive()) // For Debugging ONLY
        .layer(middleware::from_fn(logging_middleware));

    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;

    Ok(())
}