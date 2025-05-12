use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpDto {
    pub username: String,
    pub email: String,
    pub password: String,
}
