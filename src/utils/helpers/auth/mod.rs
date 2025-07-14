mod types;
pub mod google_oauth;

use chrono::{ Duration, Utc };
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use self::types::AccessTokenClaims;
use crate::constants::{
    token::MAX_AGE_ACCESS_TOKEN_SECOND
};
use crate::modules::user::{
    models::User
};
use crate::utils::errors::AppError;
use crate::utils::{
    helpers::{
        env::env_required
    },
};

pub fn generate_access_token(user: &User) -> Result<String, anyhow::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(MAX_AGE_ACCESS_TOKEN_SECOND))
        .unwrap()
        .timestamp() as usize;
    
    let claims = AccessTokenClaims {
        email: user.email.clone(),
        exp: expiration
    };

    let secret = env_required("ACCESS_TOKEN_SECRET_KEY")?;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )?;

    Ok(token)
}

pub fn decode_access_token(token: &str) -> Result<AccessTokenClaims, AppError> {
    let env_secret = env_required("ACCESS_TOKEN_SECRET_KEY")?;
    let secret_key = DecodingKey::from_secret(env_secret.as_bytes());

    match decode::<AccessTokenClaims>(token, &secret_key, &Validation::new(Algorithm::HS256)) {
        Ok(claims) => Ok(claims.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(AppError::Unauthorized),
            ErrorKind::InvalidToken => Err(AppError::Unauthorized),
            _ => {
                Err(AppError::Internal(err.into()))
            }
        }
    }
}

