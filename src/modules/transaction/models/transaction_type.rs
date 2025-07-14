use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    #[sqlx(rename = "income")]
    Income,

    #[sqlx(rename = "expense")]
    Expense,
}
