use crate::modules::auth::models::AuthProviderType;

pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub provider: AuthProviderType,
    pub provider_user_id: String,
}