use actix_web::{ middleware::from_fn, web };

use crate::middlewares::user_auth::user_auth_middleware;
use crate::modules::transaction::{
    handler::create_transaction
};

use super::handler::{
    get_user_pockets
};

pub fn config_pocket_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pockets")
            .wrap(from_fn(user_auth_middleware))
            .service(get_user_pockets)
            .service(
                web::scope("/{pocket_id}/transactions")
                    .service(create_transaction)
            )
    );
}
