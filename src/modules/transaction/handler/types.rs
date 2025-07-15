use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::modules::transaction::models::TransactionType;
use crate::utils::validators::decimal::validate_positive_decimal;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTransactionPayload {
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub name: String,
    #[validate(custom(function = "validate_positive_decimal", message = "Amount must be a positive decimal"))]
    pub amount: BigDecimal,
}
