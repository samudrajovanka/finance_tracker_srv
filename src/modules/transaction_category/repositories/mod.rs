use sqlx::{PgExecutor};
use uuid::Uuid;

use crate::modules::transaction::models::TransactionType;

pub async fn check_exist_category_by_id_and_type<'e, E>(
    executor: E,
    id: &Uuid,
    transaction_type: &TransactionType
) -> Result<bool, sqlx::Error> where E: PgExecutor<'e> {
    let exist = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM transaction_categories
            WHERE id = $1 AND type = $2::transaction_type
        )
        "#,
        id,
        transaction_type as _
    )
    .fetch_one(executor)
    .await?;

    Ok(exist.unwrap_or(false))
}