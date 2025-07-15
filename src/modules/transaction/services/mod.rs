use sqlx::{PgPool};
use uuid::Uuid;

use crate::utils::errors::AppError;
use crate::modules::pocket::{
    services::{
        update_balance_pockets
    },
    repositories::types::{
        UpdatePocketBalancePayload
    }
};
use crate::modules::transaction_category::{
    repositories::check_exist_category_by_id_and_type
};
use super::{
    repositories,
    models
};

pub async fn create_transaction(
    pool: &PgPool,
    payload: &mut repositories::types::CreateTransactionPayload
) -> Result<Uuid, AppError> {
    let category_exist = check_exist_category_by_id_and_type(
        pool,
        &payload.category_id,
        &payload.transaction_type
    ).await?;

    if !category_exist {
        return Err(
            AppError::BadRequest("Category does not exist for this transaction type".to_string())
        );
    }

    let mut tx = pool.begin().await?;
    
    payload.amount = if payload.transaction_type == models::TransactionType::Income {
        payload.amount.clone()
    } else {
        -payload.amount.clone()
    };

    let transaction = repositories::insert_transaction_to_pocket(&mut *tx, payload).await?;
    update_balance_pockets(&mut *tx, &UpdatePocketBalancePayload {
        pocket_id: payload.pocket_id,
        user_id: payload.user_id,
        amount: payload.amount.clone()
    }).await?;

    tx.commit().await?;

    Ok(transaction.id)
}