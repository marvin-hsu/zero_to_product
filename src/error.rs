use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use tracing::{error, warn};

#[derive(Debug)]
pub enum AppError {
    BadRequestError(BadRequestError),
    DatabaseError(DbErr),
    EmailClientError(reqwest::Error),
    CustomWarning(StatusCode, String),
    CustomInternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequestError(e) => {
                warn!("BadRequestError:{:?}", e);
                (StatusCode::BAD_REQUEST, e.0).into_response()
            }
            AppError::CustomWarning(status, e) => {
                warn!("CustomError:{}", e);
                (status, e).into_response()
            }
            _ => {
                error!("AppError:{:?}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred. Please try again later.".to_string(),
                )
                    .into_response()
            }
        }
    }
}

impl From<BadRequestError> for AppError {
    fn from(error: BadRequestError) -> Self {
        AppError::BadRequestError(error)
    }
}

impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        AppError::DatabaseError(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::EmailClientError(error)
    }
}

#[derive(Debug)]
pub struct BadRequestError(pub String);

impl From<String> for BadRequestError {
    fn from(error: String) -> Self {
        BadRequestError(error)
    }
}
