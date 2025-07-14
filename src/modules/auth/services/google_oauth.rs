use reqwest::Client as HttpClient;
use crate::utils::{errors::AppError, helpers::auth::google_oauth::verify_google_id_token};
use super::types;

pub async fn handle_google_callback(code: &str) -> Result<types::GoogleUserInfo, AppError> {
    let http_client = HttpClient::new();

    let client_id = std::env::var("GOOGLE_CLIENT_ID")?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")?;

    let params = [
        ("code", code),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("redirect_uri", &redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let response = http_client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::BadRequest(String::from("Failed to exchange code for token")));
    }

    let token_resp: types::GoogleTokenResponse = response.json().await?;

    let id_token = token_resp.id_token;
    let claims = verify_google_id_token(&id_token).await?;

    let user_info = types::GoogleUserInfo {
        sub: claims.sub,
        email: claims.email,
        name: claims.name,
    };
    
    Ok(user_info)
}
