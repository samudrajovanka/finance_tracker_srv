pub mod types;
pub mod google_oauth;

use sqlx::PgPool;

use super::models::{AuthProviderType};
use crate::modules::user::{
    repositories::{
        get_user_by_provider,
        create_user,
        create_user_auth_provider,
        types::{CreateAuthProviderPayload, CreateUserPayload}
    }
};
use crate::modules::pocket::{
    services::{create_pocket},
    repositories::{
        types::CreatePocketPayload
    }
};
use crate::utils::errors::AppError;
use crate::utils::helpers::auth::generate_access_token;

pub async fn login_oauth(pool: &PgPool, provider: AuthProviderType, provider_user_id: &str, payload: CreateUserPayload) -> Result<String, AppError> {
    let user = get_user_by_provider(pool, provider.clone(), provider_user_id).await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            let mut tx = pool.begin().await?;

            let new_user = create_user(
                &mut *tx,
                payload
            ).await?;

            create_user_auth_provider(
                &mut *tx,
                CreateAuthProviderPayload {
                    user_id: new_user.id,
                    provider: provider.clone(),
                    provider_user_id: provider_user_id.to_string(),
                }
            ).await?;

            create_pocket(
                &mut *tx,
                CreatePocketPayload {
                    name: String::from("Main"),
                    user_id: new_user.id
                }
            ).await?;

            tx.commit().await?;

            new_user
        },
        Err(e) => return Err(AppError::Internal(e.into())),
    };

    let access_token = generate_access_token(&user)?;
    Ok(access_token)
}