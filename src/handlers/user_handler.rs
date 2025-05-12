use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::{AppState, CreateUser};
use crate::dtos::sign_in_dto::SignInDto;
use crate::repository::user_repository::UserRepository;
use crate::dtos::sign_up_dto::SignUpDto;

pub struct UserHandler {}

impl UserHandler {
    pub async fn sign_up(State(state) : State<AppState>, Json(payload): Json<SignUpDto>) -> impl IntoResponse{
        let user_repo = UserRepository::new(state.pool.clone());
        user_repo.sign_up(payload).await.expect("Error while creating user")
    }
    
    pub async fn sign_in(State(state) : State<AppState>, Json(payload): Json<SignInDto>) -> impl IntoResponse{
        let user_repo = UserRepository::new(state.pool.clone());
        user_repo.sign_in(payload).await.expect("Error while signing in user")
    }
}