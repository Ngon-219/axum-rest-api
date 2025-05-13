use std::fmt::format;
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

    pub async fn get_to_do_list(&self, email: String) -> Result<String, sqlx::Error> {
        let existing_user = sqlx::query!("select id from users where email = $1", email)
        .fetch_optional(&self.pool)
        .await?;
        
        if existing_user.is_none() {
            return Ok(format!("User not found"));
        }
        
        let to_do_list = sqlx::query!("SELECT users.username, todo_lists.title, users.email
            FROM todo_lists
            INNER JOIN users ON todo_lists.user_id = users.id
            WHERE todo_lists.user_id = $1
        ", existing_user.unwrap().id)
        .fetch_all(&self.pool)
        .await?;
        
        if to_do_list.is_empty() {
            return Ok(format!("To do list not found"));       
        }

        let to_do_list_titles: Vec<String> = to_do_list.iter()
            .map(|todo| todo.title.clone()) 
            .collect();
        
        let result = to_do_list_titles.join(", ");

        Ok(format!("To do list found: {}", result))
    }

    pub async fn delete_to_do_list(&self, to_do_list_id: Uuid) -> Result<String, sqlx::Error> {
        let delete_to_do_list = sqlx::query!("delete from todo_lists where id = $1", to_do_list_id)
            .execute(&self.pool)
            .await?;
        
        Ok("To do list deleted".to_string())       
    }
}