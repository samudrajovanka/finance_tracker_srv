use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
}