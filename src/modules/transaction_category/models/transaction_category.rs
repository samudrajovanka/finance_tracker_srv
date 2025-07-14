use serde::Serialize;
use sqlx::{FromRow};
use uuid::Uuid;

use crate::modules::transaction::models::TransactionType;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct TransactionCategory {
    pub id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub _type: TransactionType,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
