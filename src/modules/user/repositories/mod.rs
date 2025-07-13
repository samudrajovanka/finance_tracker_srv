pub mod types;

use sqlx::{Executor, Postgres};
use crate::modules::auth::models::{AuthProvider, AuthProviderType};
use super::models::User;
use self::types::{ CreateAuthProviderPayload, CreateUserPayload };

pub async fn get_user_by_provider<'e, E>(
    executor: E,
    provider: AuthProviderType,
    provider_user_id: &str
) -> Result<Option<User>, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT u.* FROM users u
        JOIN auth_providers ap ON u.id = ap.user_id
        WHERE ap.provider_user_id = $1 AND ap.provider = $2::auth_provider_type;
        "#,
        provider_user_id,
        provider as AuthProviderType
    )
    .fetch_optional(executor)
    .await?;

    Ok(user)
}

pub async fn create_user<'e, E>(
    executor: E,
    payload: CreateUserPayload
) -> Result<User, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING *;
        "#,
        payload.name,
        payload.email
    )
    .fetch_one(executor)
    .await
}

pub async fn create_user_auth_provider<'e, E>(
    executor: E,
    payload: CreateAuthProviderPayload
) -> Result<AuthProvider, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    sqlx::query_as!(
        AuthProvider,
        r#"
        INSERT INTO auth_providers (user_id, provider, provider_user_id)
        VALUES ($1, $2::auth_provider_type, $3)
        RETURNING
            id, 
            user_id,
            provider as "provider: _",
            provider_user_id,
            created_at,
            updated_at;
        "#,
        payload.user_id,
        payload.provider as AuthProviderType,
        payload.provider_user_id,
    )
    .fetch_one(executor)
    .await
}

pub async fn get_user_by_email<'e, E>(
    executor: E,
    email: &str
) -> Result<Option<User>, sqlx::Error> where E: Executor<'e, Database = Postgres> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE email = $1;
        "#,
        email
    )
    .fetch_optional(executor)
    .await?;

    Ok(user)
}