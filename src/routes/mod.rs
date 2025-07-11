use actix_web::web;

use crate::modules::auth::route::config_auth_routes;

pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(config_auth_routes())
    );
}