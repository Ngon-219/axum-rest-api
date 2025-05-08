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

pub async fn authenticate(
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