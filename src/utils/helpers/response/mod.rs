pub mod types;

use serde::Serialize;

use crate::utils::errors::AppError;

use self::types::{ApiResponse, ApiErrorResponse};

pub fn success_response<T: Serialize>(message: &str, data: Option<T>) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        message: message.to_string(),
        data
    }
}

pub fn error_response(error: &AppError) -> ApiErrorResponse {
    let cause = match error {
        AppError::Internal(inner) => Some(inner.to_string()),
        _ => None,
    };
    
    ApiErrorResponse {
        success: false,
        message: error.to_string(),
        error_type: error.error_type(),
        cause
    }
}