use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, sqlx::Type)]
#[sqlx(type_name = "auth_provider_type")]
pub enum AuthProviderType {
    #[sqlx(rename = "google")]
    Google,
}
