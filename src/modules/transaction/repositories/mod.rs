pub mod types;

use sqlx::{Executor, Postgres};


use super::models::{
    Transaction,
    TransactionType
};
use self::types::{
    CreateTransactionPayload
};

pub async fn insert_transaction_to_pocket<'e, E>(
    executor: E,
    payload: &CreateTransactionPayload
) -> Result<Transaction, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    sqlx::query_as!(
        Transaction,
        r#"
        INSERT INTO transactions (user_id, pocket_id, category_id, transaction_type, name, amount)
        VALUES ($1, $2, $3, $4::transaction_type, $5, $6)
        RETURNING
            id,
            user_id,
            pocket_id,
            category_id,
            transaction_type as "transaction_type: _",
            name,
            amount,
            created_at,
            updated_at;
        "#,
        payload.user_id,
        payload.pocket_id,
        payload.category_id,
        payload.transaction_type.clone() as TransactionType,
        payload.name,
        payload.amount
    )
    .fetch_one(executor)
    .await
}