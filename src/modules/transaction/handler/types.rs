use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::transaction::models::TransactionType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionPayload {
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub name: String,
    pub amount: BigDecimal,
}