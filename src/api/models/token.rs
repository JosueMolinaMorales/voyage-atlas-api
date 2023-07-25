use std::{future::Future, pin::Pin};

use actix_web::FromRequest;
use anyhow::Context;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};

use super::error::{ApiError, Result};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JwtPayload {
    pub user_id: String,
    pub iss: u64,
    pub exp: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Token {
    pub token: String,
}

impl FromRequest for JwtPayload {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = core::result::Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(|value| value.to_string())
            .ok_or(ApiError::Unauthorized(anyhow::anyhow!(
                "Missing Authorization Token."
            )));

        Box::pin(async move {
            match token {
                Ok(token) => {
                    // Validate token
                    let token: TokenData<JwtPayload> = jsonwebtoken::decode(
                        &token,
                        &DecodingKey::from_secret("Secret".as_bytes()),
                        &Validation::default(),
                    )
                    .map_err(|err| ApiError::Unauthorized(anyhow::anyhow!(err.to_string())))?;
                    Ok(token.claims)
                }
                Err(err) => Err(err),
            }
        })
    }
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
