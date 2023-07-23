
use anyhow::Context;
use jsonwebtoken::{EncodingKey, Header};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::api::{CreateUser, error::{Result, ApiError}, User, token::JwtPayload, AuthUser, LoginInfo};

pub async fn register(new_user: CreateUser, conn: &PgPool) -> Result<String> {
    // Insert user into db
    let user_id = uuid::Uuid::new_v4();

    // Check if username is already taken
    let is_username_taken = sqlx::query!(
        r#"
        SELECT username FROM users WHERE username = $1
        "#,
        new_user.username
    )
    .fetch_optional(conn)
    .await
    .context("Failed to check if username is already taken.")
    .map_err(ApiError::Database)?
    .is_some();

    if is_username_taken {
        return Err(ApiError::BadRequest(anyhow::anyhow!(
            "Username already taken.".to_string()
        )));
    }

    // Check if email is already taken
    let is_email_tken = sqlx::query!(
        r#"
        SELECT email FROM users WHERE email = $1
        "#,
        new_user.email
    )
    .fetch_optional(conn)
    .await
    .context("Failed to check if email is already taken.")
    .map_err(ApiError::Database)?
    .is_some();

    if is_email_tken {
        return Err(ApiError::BadRequest(anyhow::anyhow!(
            "Email already taken.".to_string()
        )));
    }

    let hashed_pwd = pwhash::bcrypt::hash(&new_user.password)
        .context("Failed to hash password for new user.")
        .map_err(ApiError::InternalServer)?;

    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        new_user.username,
        new_user.email,
        hashed_pwd
    )
    .execute(conn)
    .await
    .context("Failed to insert new user into database.")
    .map_err(ApiError::Database)?;

    // Generate JWT
    let jwt = jsonwebtoken::encode(
        &Header::default(),
        &JwtPayload {
            user_id: user_id.to_string(),
            iss: chrono::Utc::now().timestamp() as u64,
            exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as u64,
        },
        &EncodingKey::from_secret("Secret".as_ref()),
    )
    .context("Failed to generate JWT.")
    .map_err(ApiError::InternalServer)?;

    Ok(jwt)
}

pub async fn login(login: LoginInfo, conn: &PgPool) -> Result<String> {
    // Check if user exists
    let user: Option<User> = sqlx::query!(
        r#"
        SELECT id, username, email, password FROM users WHERE email = $1
        "#,
        login.email
    )
    .fetch_optional(conn)
    .await
    .context("Failed to check if user exists.")
    .map_err(ApiError::Database)?
    .map(|row| User {
        id: row.id.to_string(),
        username: row.username,
        email: row.email,
        password: Secret::new(row.password),
    });

    let auth_user: AuthUser = if let Some(user) = user {
        // Check password
        if !pwhash::bcrypt::verify(login.password, user.password.expose_secret()) {
            return Err(ApiError::BadRequest(anyhow::anyhow!(
                "Email or Password is incorrect".to_string()
            )));
        }
        user.into()
    } else {
        return Err(ApiError::BadRequest(anyhow::anyhow!(
            "Email or Password is incorrect".to_string()
        )));
    };

    // Generate JWT
    let token = jsonwebtoken::encode(
        &Header::default(),
        &JwtPayload {
            user_id: auth_user.id,
            iss: chrono::Utc::now().timestamp() as u64,
            exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as u64,
        },
        &EncodingKey::from_secret("Secret".as_ref()),
    )
    .context("Failed to generate JWT.")
    .map_err(ApiError::InternalServer)?;

    Ok(token)
}
