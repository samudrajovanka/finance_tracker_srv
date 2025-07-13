use actix_web::web;

use crate::modules::auth::route::config_auth_routes;
use crate::modules::pocket::route::config_pocket_routes;

pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(config_auth_routes)
            .configure(config_pocket_routes)
    );
}