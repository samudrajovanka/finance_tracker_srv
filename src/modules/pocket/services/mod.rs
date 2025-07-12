use sqlx::{Executor, Postgres};

use crate::utils::errors::AppError;
use super::repositories::{
    types::CreatePocketPayload,
};
use super::models::Pocket;
use super::repositories::{
    create_pocket as create_pocket_repository
};

pub async fn create_pocket<'e, E>(
    executor: E,
    payload: CreatePocketPayload
) -> Result<Pocket, AppError> where E: Executor<'e, Database = Postgres> {
    let pocket = create_pocket_repository(executor, payload).await?;

    Ok(pocket)
}