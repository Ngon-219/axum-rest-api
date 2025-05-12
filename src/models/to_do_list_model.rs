use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::{Timestamp, Uuid};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ToDoList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}