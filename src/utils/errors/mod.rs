pub mod json_error;

use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::derive::{Display, Error};

use crate::utils::helpers::response::error_response as error_response_template;

#[derive(Debug, Error, Display)]
pub enum AppError {
    #[display("{_0}")]
    BadRequest(#[error(not(source))] String),

    #[display("{_0}")]
    NotFound(#[error(not(source))] String),

    #[display("Unauthorized access. Please log in.")]
    Unauthorized,

    #[display("An internal error occurred. Please try again later.")]
    Internal(#[error(not(source))] anyhow::Error),
}

impl AppError {
    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::Internal(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(serde_json::json!(error_response_template(self)))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::Internal(anyhow::anyhow!(err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(err))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::Internal(anyhow::anyhow!("{err}"))
    }
}
