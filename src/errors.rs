use crate::helpers::api_response::ApiResponse;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sea_orm::DbErr;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(String),
    BadRequest(String),
    UnprocessableEntity(HashMap<String, String>),
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

    pub fn unprocessable_entity(errors: HashMap<String, String>) -> Self {
        Self::UnprocessableEntity(errors)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) | AppError::Internal(msg) | AppError::BadRequest(msg) => {
                write!(f, "{}", msg)
            }
            AppError::UnprocessableEntity(_) => {
                write!(f, "Erreur de validation des données")
            }
        }
    }
}

impl From<DbErr> for AppError {
    fn from(_: DbErr) -> Self {
        AppError::Internal("Database error occurred".into())
    }
}

impl From<actix_web_validator::Error> for AppError {
    fn from(err: actix_web_validator::Error) -> Self {
        match err {
            actix_web_validator::Error::Validate(errors) => {
                let mut field_errors = HashMap::new();

                for (field, err_list) in errors.field_errors() {
                    if let Some(first_err) = err_list.first() {
                        let msg = first_err
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("Le champ {} est invalide.", field));

                        field_errors.insert(field.to_string(), msg);
                    }
                }
                AppError::UnprocessableEntity(field_errors)
            }
            _ => {
                let mut map = HashMap::new();
                map.insert("json".to_string(), "Format JSON invalide.".to_string());
                AppError::UnprocessableEntity(map)
            }
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
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
            AppError::UnprocessableEntity(errors) => HttpResponse::UnprocessableEntity().json(
                ApiResponse::<HashMap<String, String>>::unprocessable_entity(
                    "Données de formulaire invalides.",
                    errors.clone(),
                ),
            ),
        }
    }
}
