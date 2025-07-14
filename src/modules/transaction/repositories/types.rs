use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::modules::transaction::models::TransactionType;

pub struct CreateTransactionPayload {
    pub user_id: Uuid,
    pub pocket_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: TransactionType,
    pub name: String,
    pub amount: BigDecimal,
}