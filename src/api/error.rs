use std::fmt;
use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    // Database errors
    DatabaseError(String),
    NotFound(String),
    Conflict(String),

    // Validation errors
    InvalidInput(String),
    InvalidQuery(String),

    // Client errors
    Unauthorized,
    Forbidden,

    // Server errors
    InternalError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ApiError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Forbidden => write!(f, "Forbidden"),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                msg.clone(),
            ),
            ApiError::InvalidInput(msg) | ApiError::InvalidQuery(msg) => (
                StatusCode::BAD_REQUEST,
                "INVALID_REQUEST",
                msg.clone(),
            ),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "CONFLICT",
                msg.clone(),
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Unauthorized".to_string(),
            ),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Forbidden".to_string(),
            ),
            ApiError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Database operation failed".to_string(),
                )
            }
            ApiError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
                "status": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound("Resource not found".to_string())
            }
            _ => {
                tracing::error!("SQLx error: {:?}", err);
                ApiError::DatabaseError(err.to_string())
            }
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;