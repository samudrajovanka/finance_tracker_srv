use actix_web::{ web };
use super::handlers::google_oauth;

pub fn config_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::scope("/google")
                    .service(google_oauth::google_login)
                    .service(google_oauth::google_callback)
            )
    );
}
