use actix_web::{error::JsonPayloadError, HttpRequest};
use regex::Regex;
use log::error;

use super::AppError;

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let message = match &err {
        JsonPayloadError::Deserialize(e) => {
            let err_str = e.to_string();
            let cleanup_re = Regex::new(r" at line \d+ column \d+").unwrap();
            let cleaned = cleanup_re.replace(&err_str, "");
            
            format!("Invalid payload: {}", cleaned)
        },
        JsonPayloadError::ContentType => "Expected content-type application/json".to_string(),
        _ => format!("Invalid payload: {}", err),
    };

    error!("JSON error: {}", message);
    AppError::BadRequest(message).into()
}
