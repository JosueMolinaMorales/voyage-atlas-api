use anyhow::Context;
use jsonwebtoken::{EncodingKey, Header};

use super::error::{ApiError, Result};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JwtPayload {
    pub user_id: String,
    pub iss: u64,
    pub exp: u64,
}

pub fn generate_token(user_id: String) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &JwtPayload {
            user_id,
            iss: chrono::Utc::now().timestamp() as u64,
            exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as u64,
        },
        &EncodingKey::from_secret("Secret".as_ref()),
    )
    .context("Failed to generate JWT.")
    .map_err(ApiError::InternalServer)?)
}
