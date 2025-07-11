mod types;
pub mod google_oauth;

use std::env;
use chrono::{ Utc, Duration };
use jsonwebtoken::{encode, EncodingKey, Header};

use self::types::AccessTokenClaims;
use crate::modules::user::models::User;

pub fn generate_access_token(user: &User) -> Result<String, anyhow::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    let claims = AccessTokenClaims {
        email: user.email.clone(),
        exp: expiration
    };

    let secret = env::var("ACCESS_TOKEN_SECRET_KEY")?;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )?;

    Ok(token)
}

