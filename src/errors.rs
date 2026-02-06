use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use crate::models::ApiResponse;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Database(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl AppError {
    fn code(&self) -> i32 {
        match self {
            AppError::BadRequest(_) => 40001,
            AppError::NotFound(_) => 40401,
            AppError::Database(_) => 50001,
            AppError::Internal(_) => 50000,
        }
    }

    fn message(&self) -> String {
        match self {
            AppError::BadRequest(msg)
            | AppError::NotFound(msg)
            | AppError::Database(msg)
            | AppError::Internal(msg) => msg.clone(),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status()
    }

    fn error_response(&self) -> HttpResponse {
        let body = ApiResponse::<()>::err(
            self.code(),
            self.message(),
        );

        HttpResponse::build(self.status())
            .json(body)
    }
}
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

