use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::dtos::create_to_do_list_dto::CreateToDoListDto;
use crate::dtos::update_to_do_list_dto::UpdateToDoList;

pub struct ToDoListRepository {
    pool: PgPool
}

impl ToDoListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_to_do_list(&self, payload: CreateToDoListDto ) -> Result<String, sqlx::Error> {
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1",
            payload.user_email
        )
            .fetch_optional(&self.pool)
            .await?;

        println!("{:?}", existing_user);

        if existing_user.is_none() {
            return Ok("User not found".to_string());
        }

        let existing_to_do_list = sqlx::query!(
            "SELECT id FROM todo_lists WHERE id = $1",
            payload.to_do_list_id
        )
            .fetch_optional(&self.pool)
            .await?;

        if !existing_to_do_list.is_none() {
            return Ok("To do list was existing".to_string());
        }

        let create_to_do_list = sqlx::query!(
            "INSERT INTO todo_lists (user_id, title) VALUES ($1, $2)",
            existing_user.unwrap().id,
            payload.title,
        )
            .execute(&self.pool)
            .await?;

        Ok("To do list created".to_string())
    }

    pub async fn update_to_do_list(&self, payload: UpdateToDoList) -> Result<String, sqlx::Error> {
        let existing_to_do_list = sqlx::query!(
            "SELECT id FROM todo_lists WHERE id = $1",
            payload.to_do_list_id
        )
            .fetch_optional(&self.pool)
            .await?;

        if existing_to_do_list.is_none() {
            return Ok("To do list not found".to_string());
        }

        let update_to_do_list = sqlx::query!(
            "UPDATE todo_lists SET title = $1 WHERE id = $2",
            payload.title,
            payload.to_do_list_id
        )
            .execute(&self.pool)
            .await?;

        Ok("To do list updated".to_string())
    }

    pub async fn get_to_do_list(&self, payload: Uuid) -> Result<Json<Vec<String>>, sqlx::Error> {
        // let email_of_user =
        let get_to_do_list = sqlx::query!("select  username from todo_lists inner join users on todo_lists.user_id = users.id where todo_lists.id = $1", payload)
            .fetch_all(&self.pool)
            .await?;

        let usernames: Vec<String> = get_to_do_list.into_iter().map(|row| row.username).collect();

        Ok(Json(usernames))
    }
}