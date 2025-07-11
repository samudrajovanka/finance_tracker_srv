use actix_web::{ get, HttpResponse, Responder, web, cookie };
use sqlx::PgPool;
use url::Url;
use std::env;

use super::super::services::{google_oauth, login_oauth};
use super::super::models::{AuthProviderType};
use crate::modules::user::repositories::{
    types::{CreateUserPayload},
};
use crate::utils::errors::AppError;
use crate::utils::helpers::env::env_required;
use super::types::OAuthCallbackQuery;

#[get("/login")]
pub async fn google_login() -> impl Responder {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");

    let auth_url = Url::parse_with_params(
        "https://accounts.google.com/o/oauth2/v2/auth",
        &[
            ("client_id", client_id.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("response_type", "code"),
            ("scope", "openid email profile"),
            ("access_type", "offline"),
            ("prompt", "consent"),
        ],
    ).unwrap();

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/callback")]
pub async fn google_callback(
    query: web::Query<OAuthCallbackQuery>,
    pool: web::Data<PgPool>
) -> Result<impl Responder, AppError> {
    let google_user = google_oauth::handle_google_callback(&query.code)
        .await?;

    let payload = CreateUserPayload {
        name: google_user.name.clone(),
        email: google_user.email.clone(),
    };  

    let access_token = login_oauth(&pool, AuthProviderType::Google, &google_user.sub, payload)
        .await?;

    let is_secure = env::var("APP_ENV").unwrap_or_default() == "production";
    let cookie = cookie::Cookie::build("access_token", access_token.clone())
        .path("/")
        .http_only(true)
        .secure(is_secure)
        .max_age(time::Duration::hours(24))
        .finish();

    let frontend_redirect_url = env_required("REDIRECT_OAUTH_AFTER_LOGIN_URL")?;

    Ok(
        HttpResponse::Found()
            .append_header(("Location", frontend_redirect_url))
            .cookie(cookie)
            .finish()
    )
}