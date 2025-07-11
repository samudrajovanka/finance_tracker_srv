use actix_web::{HttpRequest, Responder, ResponseError};

use crate::utils::errors::AppError;

pub async fn not_found(_req: HttpRequest) -> impl Responder {
    AppError::NotFound(String::from("The requested resource was not found."))
        .error_response()
}