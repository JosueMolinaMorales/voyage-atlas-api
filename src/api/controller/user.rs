use anyhow::Context;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::api::{
    database,
    error::{ApiError, Result},
    token::{self},
    AuthUser, CreateUser, LoginInfo, User,
};

pub async fn register(new_user: CreateUser, conn: &PgPool) -> Result<String> {
    // Insert user into db
    let user_id = uuid::Uuid::new_v4();

    // Check if username is already taken
    let is_username_taken = database::get_user_by_username(conn, &new_user.username)
        .await?
        .is_some();

    if is_username_taken {
        return Err(ApiError::BadRequest(anyhow::anyhow!(
            "Username already taken.".to_string()
        )));
    }

    // Check if email is already taken
    let is_email_taken = database::get_user_by_email(conn, &new_user.email)
        .await?
        .is_some();

    if is_email_taken {
        return Err(ApiError::BadRequest(anyhow::anyhow!(
            "Email already taken.".to_string()
        )));
    }

    let hashed_pwd = pwhash::bcrypt::hash(&new_user.password)
        .context("Failed to hash password for new user.")
        .map_err(ApiError::InternalServer)?;

    database::insert_user(conn, &user_id, hashed_pwd, &new_user).await?;

    // Generate JWT
    let jwt = token::generate_token(user_id.to_string())?;

    Ok(jwt)
}

pub async fn login(login: LoginInfo, conn: &PgPool) -> Result<String> {
    // Check if user exists
    let user: Option<User> = database::get_user_by_email(conn, &login.email).await?;

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
    let token = token::generate_token(auth_user.id)?;

    Ok(token)
}
