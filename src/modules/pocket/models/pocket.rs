use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::{FromRow};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Pocket {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub balance: BigDecimal,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
