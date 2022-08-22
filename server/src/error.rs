use thiserror::Error;
use sqlx;

#[derive(Debug, Error, Clone)]
pub enum Error{
    #[error("Internal error")]
    Internal(String),
}

impl std::convert::From<std::num::ParseIntError> for Error{
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Internal(format!("cannot parse to int :{}",err.to_string()))
    }
}

impl std::convert::From<sqlx::Error> for Error{
    fn from(e: sqlx::Error) -> Self {
        Error::Internal(format!("sqlx error :{}",e.to_string()))
    }
}

impl std::convert::From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error::Internal(format!("sqlx migrate error :{}",err.to_string()))
    }
}