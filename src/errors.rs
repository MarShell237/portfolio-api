use crate::helpers::api_response::ApiResponse;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(String),
    BadRequest(String),
}

impl AppError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    // pub fn internal(msg: impl Into<String>) -> Self {
    //     Self::Internal(msg.into())
    // }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) | AppError::Internal(msg) | AppError::BadRequest(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl From<DbErr> for AppError {
    fn from(_: DbErr) -> Self {
        AppError::Internal("Database error occurred".into())
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(ApiResponse::<()>::not_found(msg))
            }
            AppError::Internal(msg) => HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::internal_server_error(msg)),
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ApiResponse::<()>::bad_request(msg))
            }
        }
    }
}
