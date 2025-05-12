use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}