pub mod types;
pub mod google_oauth;

use sqlx::PgPool;

use crate::modules::user::{
    services::{
        types::{
            CreateUserPayload
        },
        get_user_by_provider,
        create_user_with_pocket
    }
};
use crate::utils::errors::AppError;
use crate::utils::helpers::auth::generate_access_token;

pub async fn login_oauth(pool: &PgPool, payload: CreateUserPayload) -> Result<String, AppError> {
    let user = get_user_by_provider(pool, payload.provider.clone(), payload.provider_user_id.as_str()).await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            let mut tx = pool.begin().await?;

            let new_user = create_user_with_pocket(
                &mut tx,
                payload
            ).await?;

            tx.commit().await?;

            new_user
        },
        Err(e) => return Err(AppError::Internal(e.into())),
    };

    let access_token = generate_access_token(&user)?;
    Ok(access_token)
}