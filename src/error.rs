use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request")]
    BadRequest(String),

    #[error("Entity not found")]
    NotFound(String),

    #[error("Internal server error")]
    InternalServerError(String),

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Authentication required")]
    Unauthorized(String),

    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("Connection pool error")]
    Pool(#[from] r2d2::Error),

    #[error("Multipart parsing error")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("I/O error")]
    IO(#[from] std::io::Error),
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
            AppError::Multipart(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AppError::Multipart(e) => {
                error!("Multipart error occurred: {}", e);
                "Multipart error occurred".to_string()
            }
            AppError::IO(e) => {
                error!("I/O error occurred: {}", e);
                "I/O error occurred".to_string()
            }
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code(), Json(self.message())).into_response()
    }
}
//TODO: сделать как здесь https://github.com/launchbadge/realworld-axum-sqlx/blob/main/src/http/error.rs
