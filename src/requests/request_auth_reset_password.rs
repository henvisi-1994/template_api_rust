use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthResetPasswordRequest {
    pub current_password: String,
    pub new_password: String,
}
