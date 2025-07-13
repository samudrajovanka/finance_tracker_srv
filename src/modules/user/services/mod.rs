pub mod types;

use sqlx::{PgPool, Postgres, Transaction};

use crate::constants::pocket::DEFAULT_POCKET_NAME;
use crate::modules::auth::models::AuthProviderType;
use crate::modules::pocket::{
    repositories::{
        types::CreatePocketPayload
    },
    services::{
        create_pocket
    }
};
use crate::utils::errors::AppError;
use super::{
    models::User,
    repositories
};
use self::types::CreateUserPayload;

pub async fn get_user_by_provider(
    pool: &PgPool,
    provider: AuthProviderType,
    provider_user_id: &str
) -> Result<Option<User>, AppError> {
    let user = repositories::get_user_by_provider(pool, provider, provider_user_id)
        .await?;

    Ok(user)
}

pub async fn create_user_with_pocket(
    tx: &mut Transaction<'_, Postgres>,
    payload: CreateUserPayload
) -> Result<User, AppError> {
    let user = repositories::create_user(&mut **tx, repositories::types::CreateUserPayload {
        name: payload.name,
        email: payload.email
    })
        .await?;

    repositories::create_user_auth_provider(&mut **tx, repositories::types::CreateAuthProviderPayload {
        user_id: user.id,
        provider: payload.provider,
        provider_user_id: payload.provider_user_id
    })
    .await?;

    create_pocket(&mut **tx, CreatePocketPayload {
        user_id: user.id,
        name: String::from(DEFAULT_POCKET_NAME)
    }).await?;

    Ok(user)
}

pub async fn get_user_by_email(
    pool: &PgPool,
    email: &str
) -> Result<Option<User>, AppError> {
    let user = repositories::get_user_by_email(pool, email)
        .await?;

    Ok(user)
}