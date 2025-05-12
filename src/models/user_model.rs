use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx::types::time::Time;
use uuid::{Timestamp, Uuid};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
