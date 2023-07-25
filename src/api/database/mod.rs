use anyhow::Context;
use secrecy::Secret;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::User;

use super::{
    error::{ApiError, Result},
    CreateUser, Post,
};

pub async fn get_user_by_id(conn: &PgPool, user_id: &Uuid) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get user by id.")
    .map_err(ApiError::Database)?
    .map(|user| User {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        password: Secret::new(user.password),
    });

    Ok(user)
}

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
        email.to_lowercase()
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

pub async fn get_users_posts(conn: &PgPool, user_id: &Uuid) -> Result<Vec<Post>> {
    let posts = sqlx::query!(
        r#"
        SELECT *
        FROM posts
        WHERE author = $1
        "#,
        user_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get user's posts.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|post| Post {
        id: post.id.to_string(),
        title: post.title,
        location: post.location,
        content: post.content,
        author: post.author.to_string(),
        created_at: post.created_at.timestamp(),
    })
    .collect::<Vec<Post>>();

    Ok(posts)
}
