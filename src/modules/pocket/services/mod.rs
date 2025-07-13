use sqlx::{Executor, Postgres};
use uuid::Uuid;

use crate::utils::errors::AppError;
use super::{
    models::Pocket,
    repositories,
};

pub async fn create_pocket<'e, E>(
    executor: E,
    payload: repositories::types::CreatePocketPayload
) -> Result<Pocket, AppError> where E: Executor<'e, Database = Postgres> {
    let pocket = repositories::create_pocket(executor, payload).await?;

    Ok(pocket)
}

pub async fn get_user_pockets<'e, E>(
    executor: E,
    user_id: Uuid
) -> Result<Vec<Pocket>, AppError> where E: Executor<'e, Database = Postgres> {
    let pockets = repositories::get_pockets_by_user_id(executor, user_id).await?;

    Ok(pockets)
}