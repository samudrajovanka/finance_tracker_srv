use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::{FromRow};
use uuid::Uuid;

use super::TransactionType;

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pocket_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub name: String,
    pub amount: BigDecimal,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
