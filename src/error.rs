use axum::{Json, http::StatusCode};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalServerError(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Pool(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            AppError::BadRequest(msg) => {
                error!("Bad Request: {}", msg);
                "Bad request".to_string()
            }
            AppError::NotFound(msg) => {
                error!("Not Found: {}", msg);
                "Resource not found".to_string()
            }
            AppError::InternalServerError(msg) => {
                error!("Internal Server Error: {}", msg);
                "Internal server error".to_string()
            }
            AppError::Forbidden(msg) => {
                error!("Forbidden: {}", msg);
                "Forbidden access".to_string()
            }
            AppError::Unauthorized(msg) => {
                error!("Unauthorized: {}", msg);
                "Unauthorized access".to_string()
            }
            AppError::Database(e) => {
                error!("Database error occurred: {}", e);
                "Database error occurred".to_string()
            }
            AppError::Pool(e) => {
                error!("Connection pool error: {}", e);
                "Connection pool error occurred".to_string()
            }
        }
    }
}

pub async fn handle_error(err: AppError) -> (StatusCode, Json<ErrorResponse>) {
    let response = ErrorResponse {
        code: err.status_code().as_u16(),
        message: err.message(),
    };
    (err.status_code(), Json(response))
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}
