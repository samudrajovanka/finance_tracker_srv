use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GoogleClaims {
    pub sub: String,
    pub email: String,
    pub name: String
}

#[derive(Debug, Serialize)]
pub struct AccessTokenClaims {
    pub email: String,
    pub exp: usize
}