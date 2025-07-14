mod types;

use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::user::{
    models::User
};
use crate::utils::errors::AppError;
use crate::utils::helpers::response::success_response;
use super::{
    services,
    repositories
};

#[post("")]
pub async fn create_transaction(
    pool: web::Data<PgPool>,
    logged_user: web::ReqData<User>,
    pocket_id: web::Path<Uuid>,
    payload: web::Json<types::CreateTransactionPayload>
) -> Result<impl Responder, AppError> {
    let transaction_id = services::create_transaction(&pool, &repositories::types::CreateTransactionPayload {
        pocket_id: *pocket_id,
        user_id: logged_user.id,
        category_id: payload.category_id,
        transaction_type: payload.transaction_type.clone(),
        name: payload.name.clone(),
        amount: payload.amount.clone()
    }).await?;

    Ok(
        HttpResponse::Created()
            .json(
                success_response(
                    "Transaction created successfully",
                    Some(serde_json::json!({
                        "transaction_id": transaction_id.to_string()
                    }))
                )
            )
    )
}