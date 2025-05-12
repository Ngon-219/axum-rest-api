use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateToDoList {
    pub title: String,
    pub user_email: String,
    pub to_do_list_id: uuid::Uuid,
}