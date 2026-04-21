use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;
use std::time::Instant;
use axum::body::Body;

pub async fn logging_middleware(ConnectInfo(addr): ConnectInfo<SocketAddr>, req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let uri = req.uri().to_string();

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    let status = response.status().as_u16();

    match status {
        200..=399 => {},
        400..=499 => {
            tracing::warn!(
                target: "api",
                method = %method,
                uri = %uri,
                remote_addr = %addr,
                status = %status,
                duration_ms = elapsed.as_millis(),
                "Response:"
            );
        }
        _ => {
            tracing::error!(
                target: "api",
                method = %method,
                uri = %uri,
                remote_addr = %addr,
                status = %status,
                duration_ms = elapsed.as_millis(),
                "Response:"
            );
        }
    }

    response
}