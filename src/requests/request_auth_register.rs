use serde::Deserialize;

/// Minimal register payload for the template.
#[derive(Deserialize)]
pub struct AuthRegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}
