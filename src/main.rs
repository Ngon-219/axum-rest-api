mod middlewares;
mod handlers;
mod models;
mod repository;
mod dtos;

use axum::{
    extract::Path,
    routing::get,
    routing::post,
    Json,
    Router,
};
use axum::routing::patch;
use serde::Deserialize;
use crate::handlers::jsonwebtoken::create_jwt;
use sqlx::postgres::PgPoolOptions;
use crate::handlers::user_handler::UserHandler;
use crate::handlers::to_do_list_handler::ToDoListHandler;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    println!("Connected to database");

    let app_state = AppState { pool };
    
    let authentication = Router::new()
        .route("/sign-up", post(UserHandler::sign_up))
        .route("/sign-in", post(UserHandler::sign_in));
    
    let to_do_list_routes = Router::new()
        .route("/", post(ToDoListHandler::create_to_do_list))
        .route("/", patch(ToDoListHandler::update_to_do_list))
        .layer(axum::middleware::from_fn(middlewares::authentication::authenticate));

    let user_routes = Router::new()
        .route("/:id", get(handler))
        .layer(axum::middleware::from_fn(middlewares::authentication::authenticate))
        .route("/", post(create_user));
    let team_routes = Router::new().route("/team/:id", get(handler));
    let full_captures_param = Router::new().route("/:version/:id", get(handler_full_param));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/", team_routes)
        .nest("/handle", full_captures_param)
        .nest("/authentication", authentication)
        .nest("/to-do-list", to_do_list_routes)
        .with_state(app_state.clone());

    let app = Router::new().nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// handle param
async fn handler(Path(id): Path<String>) -> String {
    format!("you passed id: {}", id)
}

//handle full param
async fn handler_full_param(Path((version, id)): Path<(String, String)>) -> String {
    format!("You passed version: {}, id: {}", version, id)
}

//handle body
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u8,
}

async fn create_user(Json(payload): Json<CreateUser>) -> String {
    let jwt = create_jwt(payload.name);
    format!("Received jwt: {}", jwt)
}
