pub mod types;

use sqlx::{Executor, Postgres};

use super::models::Pocket;
use self::types::CreatePocketPayload;

pub async fn create_pocket<'e, E>(
    executor: E,
    payload: CreatePocketPayload
) -> Result<Pocket, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    sqlx::query_as!(
        Pocket,
        r#"
        INSERT INTO pockets (user_id, name)
        VALUES ($1, $2)
        RETURNING *;
        "#,
        payload.user_id,
        payload.name
    )
    .fetch_one(executor)
    .await
}