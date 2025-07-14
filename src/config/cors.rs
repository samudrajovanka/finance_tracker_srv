use actix_cors::Cors;
use actix_web::http;

use crate::utils::helpers::env::env_required;

pub fn setup_cors() -> Cors {
    let allowed_origins: Vec<String> = env_required("CORS_ALLOWED_ORIGINS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    Cors::default()
        .allowed_origin_fn(move |origin, _req_head| {
            allowed_origins.iter().any(|allowed_origin| allowed_origin == origin.to_str().unwrap_or_default())
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}