use axum::{
    extract::Path, http::{Request, StatusCode},
    middleware::Next,
    routing::get,
    routing::post,
    Json,
    Router,
};
// use std::collections::HashMap;
use serde::Deserialize;

use axum::http::header::AUTHORIZATION;

#[tokio::main]
async fn main() {
    let user_routes = Router::new()
        .route("/:id", get(handler))
        .route("/", post(create_user))
        .layer(axum::middleware::from_fn(authenticate));
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
    format!("Received user: {}, age: {}", payload.name, payload.age)
}

async fn authenticate(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if auth_header == "Bearer ngon" {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
