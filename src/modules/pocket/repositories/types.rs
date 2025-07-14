use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct CreatePocketPayload {
    pub user_id: Uuid,
    pub name: String,
}

pub struct UpdatePocketBalancePayload {
    pub user_id: Uuid,
    pub pocket_id: Uuid,
    pub amount: BigDecimal,
}