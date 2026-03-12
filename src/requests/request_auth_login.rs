use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
}
