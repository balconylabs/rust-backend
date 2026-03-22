use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
