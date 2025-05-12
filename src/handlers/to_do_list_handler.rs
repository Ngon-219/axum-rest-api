use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::dtos::create_to_do_list_dto::CreateToDoListDto;
use crate::repository::to_do_list_repository::ToDoListRepository;
use crate::dtos::update_to_do_list_dto::UpdateToDoList;

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
    
    // pub async fn get_to_do_list(State(state) : State<AppState>, )
}