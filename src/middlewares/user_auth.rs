use actix_web::{
    body::MessageBody,
    dev::{
        ServiceRequest, ServiceResponse
    },
    middleware::Next, web, HttpMessage,
    Error
};

use crate::{constants::token::COOKIE_ACCESS_TOKEN_KEY, modules::user::services::get_user_by_email};
use crate::utils::{
    errors::AppError,
    helpers::auth::decode_access_token
};

pub async fn user_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token = req
        .cookie(COOKIE_ACCESS_TOKEN_KEY)
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| AppError::Unauthorized)?;

    
    let claims = decode_access_token(&token)?;

    let pool = req
        .app_data::<web::Data<sqlx::PgPool>>()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!(format!("DB Pool not available"))))?;

    let user = get_user_by_email(pool, &claims.email).await?;

    match user {
        Some(user) => {
            req.extensions_mut().insert(user);
            next.call(req).await
        },
        None => return Err(AppError::Unauthorized.into()),
    }
}