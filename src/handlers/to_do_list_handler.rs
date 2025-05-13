use std::fmt::Debug;
use axum::extract::{Path, State};
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::http::uri::Authority;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use crate::AppState;
use crate::dtos::create_to_do_list_dto::CreateToDoListDto;
use crate::repository::to_do_list_repository::ToDoListRepository;
use crate::dtos::update_to_do_list_dto::UpdateToDoList;
use crate::handlers::jsonwebtoken::{decode_jwt, get_user_email_via_access_token};
use serde::Deserialize;
use uuid::Uuid;
#[derive(Deserialize)]
struct ToDoListIdPayload {
    to_do_list_id: Uuid,
}

pub struct ToDoListHandler {}

impl ToDoListHandler {
    pub async fn create_to_do_list(State(state): State<AppState>, Json(payload) : Json<CreateToDoListDto>) -> impl IntoResponse{
        let to_do_list_repo = ToDoListRepository::new(state.pool.clone());
        let message  = to_do_list_repo.create_to_do_list(payload).await.expect("Failed to create to do list");
        return message;
    }
    
    pub async fn update_to_do_list(State(state) : State<AppState>, Json(payload) : Json<UpdateToDoList>) -> impl IntoResponse{
        let to_do_list_repo = ToDoListRepository::new(state.pool.clone());
        let message  = to_do_list_repo.update_to_do_list(payload).await.expect("Failed to update to do list");
        return message;
    }
    
    pub async fn get_to_do_list(State(state) : State<AppState>, Extension(email) : Extension<String>) -> impl IntoResponse{
        let to_do_list_repo = ToDoListRepository::new(state.pool.clone());
        let message = to_do_list_repo.get_to_do_list(email).await.expect("Failed to get to do list");
        return message;
    }
    
    pub async fn delete_to_do_list(State(state) : State<AppState>, Path(id): axum::extract::Path<Uuid>) -> impl IntoResponse{
        let to_do_list_repo = ToDoListRepository::new(state.pool.clone());
        let message = to_do_list_repo.delete_to_do_list(id).await.expect("Failed to delete to do list");
    }
}