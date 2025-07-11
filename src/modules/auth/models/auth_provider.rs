use super::auth_provider_type::AuthProviderType;

use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct AuthProvider {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: AuthProviderType,
    pub provider_user_id: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
