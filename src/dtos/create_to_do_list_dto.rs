use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateToDoListDto {
    pub title: String,
    pub user_email: String,
    pub to_do_list_id: uuid::Uuid,
}