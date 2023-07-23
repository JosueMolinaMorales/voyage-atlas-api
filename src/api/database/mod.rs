use anyhow::Context;
use secrecy::Secret;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::User;

use super::{
    error::{ApiError, Result},
    CreateUser,
};

pub async fn get_user_by_username(conn: &PgPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get user by username.")
    .map_err(ApiError::Database)?
    .map(|user| User {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        password: Secret::new(user.password),
    });

    Ok(user)
}

pub async fn get_user_by_email(conn: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get user by email.")
    .map_err(ApiError::Database)?
    .map(|user| User {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        password: Secret::new(user.password),
    });

    Ok(user)
}

pub async fn insert_user(
    conn: &PgPool,
    user_id: &Uuid,
    password: String,
    user: &CreateUser,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        user.username,
        user.email,
        password
    )
    .execute(conn)
    .await
    .context("Failed to insert new user into database.")
    .map_err(ApiError::Database)?;

    Ok(())
}
