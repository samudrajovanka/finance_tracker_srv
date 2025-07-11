use actix_web::{ web, Scope };
use super::handlers::google_oauth;

pub fn config_auth_routes() -> Scope {
    web::scope("/auth")
        .service(
            web::scope("/google")
                .service(google_oauth::google_login)
                .service(google_oauth::google_callback)
        )
}
