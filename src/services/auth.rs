use anyhow::{anyhow, Context, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Claims that the app uses internally after validating a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User id.
    pub user_id: Uuid,
    /// Email is optional because not every token guarantees it.
    pub email: Option<String>,
    /// Expiration (unix timestamp).
    pub exp: usize,
}

/// Raw claims as they come from the JWT payload.
#[derive(Debug, Serialize, Deserialize)]
struct RawClaims {
    #[serde(default)]
    pub sub: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub exp: usize,
}

fn jwt_secret() -> Result<String> {
    std::env::var("JWT_SECRET").map_err(|_| anyhow!("JWT_SECRET must be set"))
}

/// Validate and decode a JWT using the project's JWT secret.
pub fn decode_token(token: &str) -> Result<Claims> {
    let secret = jwt_secret()?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<RawClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .context("failed to decode JWT")?;

    let user_id_raw = data
        .claims
        .user_id
        .clone()
        .or_else(|| data.claims.sub.clone())
        .ok_or_else(|| anyhow!("missing user id claim"))?;
    let user_id = Uuid::parse_str(&user_id_raw).context("invalid user id")?;

    Ok(Claims {
        user_id,
        email: data.claims.email,
        exp: data.claims.exp,
    })
}
