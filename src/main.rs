mod middlewares;
mod handlers;
use axum::{
    extract::Path,
    routing::get,
    routing::post,
    Json,
    Router,
};
use serde::Deserialize;
use crate::handlers::jsonwebtoken::create_jwt;

#[tokio::main]
async fn main() {
    let user_routes = Router::new()
        .route("/:id", get(handler))
        .layer(axum::middleware::from_fn(middlewares::authentication::authenticate))
        .route("/", post(create_user));
    let team_routes = Router::new().route("/team/:id", get(handler));
    let full_captures_param = Router::new().route("/:version/:id", get(handler_full_param));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/", team_routes)
        .nest("/handle", full_captures_param);

    let app = Router::new().nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
    let jwt = create_jwt(&payload.name);
    format!("Received user: {}, age: {}, jwt: {}", payload.name, payload.age, jwt)
}
