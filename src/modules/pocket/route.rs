use actix_web::{ middleware::from_fn, web };

use crate::middlewares::user_auth::user_auth_middleware;

use super::handler::{
    get_user_pockets
};

pub fn config_pocket_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pockets")
            .wrap(from_fn(user_auth_middleware))
            .service(get_user_pockets)
    );
}
