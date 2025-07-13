pub mod types;

use sqlx::{Executor, Postgres};
use uuid::Uuid;

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

pub async fn get_pockets_by_user_id<'e, E>(
    executor: E,
    user_id: Uuid
) -> Result<Vec<Pocket>, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    sqlx::query_as!(
        Pocket,
        r#"
        SELECT * FROM pockets
        WHERE user_id = $1;
        "#,
        user_id
    )
    .fetch_all(executor)
    .await
}