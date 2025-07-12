use uuid::Uuid;

pub struct CreatePocketPayload {
    pub user_id: Uuid,
    pub name: String,
}