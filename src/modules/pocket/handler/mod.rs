use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::utils::errors::AppError;
use crate::modules::user::{
    models::User
};
use crate::utils::helpers::response::success_response;
use super::{
    services
};

#[get("")]
pub async fn get_user_pockets(
    pool: web::Data<PgPool>,
    logged_user: web::ReqData<User>
) -> Result<impl Responder, AppError> {
    let pockets = services::get_user_pockets(&**pool, logged_user.id).await?;

    Ok(
        HttpResponse::Ok()
            .json(success_response("Success get user pockets", Some(pockets)))
    )
}