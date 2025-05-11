use axum::{
    http::{Request, StatusCode},
    middleware::Next,
};
use axum::http::header::AUTHORIZATION;
use crate::handlers::jsonwebtoken::decode_jwt;

pub async fn authenticate(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        let token = auth_header
            .to_str()
            .ok()
            .and_then(|s| s.split(' ').nth(1))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let _token_data = decode_jwt(token.to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?;
        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}