pub mod types;

use sqlx::PgPool;
use crate::modules::auth::models::{AuthProvider, AuthProviderType};
use super::models::User;
use self::types::{ CreateAuthProviderPayload, CreateUserPayload };

pub async fn get_user_by_provider(pool: &PgPool, provider: AuthProviderType, provider_user_id: &str) -> Result<Option<User>, sqlx::Error> {
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
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, payload: CreateUserPayload) -> Result<User, sqlx::Error> {
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
    .fetch_one(pool)
    .await
}

pub async fn create_user_auth_provider(pool: &PgPool, payload: CreateAuthProviderPayload) -> Result<AuthProvider, sqlx::Error> {
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
    .fetch_one(pool)
    .await
}