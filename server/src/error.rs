use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;
use thiserror::Error;
use sqlx;

#[derive(Debug, Error, Clone)]
pub enum MyError{
    #[error("Internal error")]
    Internal(String),
    #[error("Not Found error")]
    NotFound(String),
}

impl std::convert::From<std::num::ParseIntError> for MyError{
    fn from(err: std::num::ParseIntError) -> Self {
        MyError::Internal(format!("cannot parse to int :{}",err.to_string()))
    }
}

impl std::convert::From<sqlx::Error> for MyError{
    fn from(e: sqlx::Error) -> Self {
        MyError::Internal(format!("sqlx error :{}",e.to_string()))
    }
}

impl std::convert::From<sqlx::migrate::MigrateError> for MyError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        MyError::Internal(format!("sqlx migrate error :{}",err.to_string()))
    }
}

impl std::convert::From<uuid::Error> for MyError {
    fn from(err: uuid::Error) -> Self {
        MyError::Internal(format!("UUID conversion error :{}",err.to_string()))
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            MyError::NotFound(s) => {
                (StatusCode::NOT_FOUND, 
                    format!("Obects not found: {}", s))
            }
            MyError::Internal(s) => {
                (StatusCode::UNPROCESSABLE_ENTITY, format!("Internal error: {}", s))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}