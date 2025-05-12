use sqlx::{PgPool};
use crate::dtos::sign_up_dto::SignUpDto;
use crate::dtos::sign_in_dto::SignInDto;
use crate::handlers::jsonwebtoken::create_jwt;
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn sign_up(&self, user: SignUpDto) -> Result<String, sqlx::Error> {
        let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        user.email
        )
        .fetch_optional(&self.pool)
        .await?;

        if existing_user.is_some() {
            return Ok("User already exists".to_string());
        }

        let _ = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id",
        user.username,
        user.email,
        user.password
    )
            .fetch_one(&self.pool)
            .await?;

        Ok("User created".to_string())
    }
    
    pub async fn sign_in(&self, user: SignInDto) -> Result<String, sqlx::Error> {
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = $1 AND password = $2",
            user.email, user.password
        )
            .fetch_optional(&self.pool)
            .await?;
        
        if existing_user.is_some() {
            let access_token = create_jwt(user.email);
            Ok(format!("access_token: {}", access_token))
        } else { 
            Ok("Invalid credentials".to_string())
        }
    }
}