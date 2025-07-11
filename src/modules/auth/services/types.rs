use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub id_token: String,
}

#[derive(Debug, Serialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
}