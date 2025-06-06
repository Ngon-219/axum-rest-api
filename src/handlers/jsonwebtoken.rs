use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey, TokenData, decode, DecodingKey, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::StatusCode;
use axum::http::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub fn create_jwt(email: String) -> String {
    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not found");
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60 * 7; // token sống 7 ngày

    let claims = Claims {
        sub: email.to_string(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not found");
    let decode = decode(&jwt_token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default());
    println!("{:?}", decode);
    decode.map_err(|_| StatusCode::UNAUTHORIZED)
}

pub fn get_user_email_via_access_token(req: Request<axum::body::Body>) -> String {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap().split(" ").nth(1).unwrap();
    let claims = decode_jwt(token.to_string()).unwrap().claims;
    claims.sub
}
