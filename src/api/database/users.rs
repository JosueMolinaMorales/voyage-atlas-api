use anyhow::Context;
use secrecy::Secret;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::{
    error::{ApiError, Result},
    AuthUser, CreateUser, Post, User,
};

pub async fn get_user_by_id(conn: &PgPool, user_id: &Uuid) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password, first_name, last_name, description
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
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
        password: Secret::new(user.password),
    });

    Ok(user)
}

pub async fn get_user_by_username(conn: &PgPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password, first_name, last_name, description
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
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
        password: Secret::new(user.password),
    });

    Ok(user)
}

pub async fn get_user_by_email(conn: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, password, first_name, last_name, description
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
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
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
        INSERT INTO users (id, username, first_name, last_name, description, email, password)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user_id,
        user.username.to_lowercase(),
        user.first_name,
        user.last_name,
        user.description,
        user.email.to_lowercase(),
        password
    )
    .execute(conn)
    .await
    .context("Failed to insert new user into database.")
    .map_err(ApiError::Database)?;

    Ok(())
}

pub async fn follow_user(conn: &PgPool, follower_id: &Uuid, followed_id: &Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users_followers (user_id, follower_id)
        VALUES ($1, $2)
        "#,
        followed_id,
        follower_id
    )
    .execute(conn)
    .await
    .context("Failed to insert new follower into database.")
    .map_err(ApiError::Database)?;
    Ok(())
}

pub async fn is_following(conn: &PgPool, follower_id: &Uuid, followed_id: &Uuid) -> Result<bool> {
    let is_following = sqlx::query!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users_followers
            WHERE user_id = $1 AND follower_id = $2
        ) AS "is_following!"
        "#,
        followed_id,
        follower_id
    )
    .fetch_one(conn)
    .await
    .context("Failed to check if user is following.")
    .map_err(ApiError::Database)?
    .is_following;

    Ok(is_following)
}

pub async fn get_followers(conn: &PgPool, user_id: &Uuid) -> Result<Vec<AuthUser>> {
    let followers = sqlx::query!(
        r#"
        SELECT users.id, users.username, users.email, users.first_name, users.last_name, users.description
        FROM users
        INNER JOIN users_followers ON users.id = users_followers.follower_id
        WHERE users_followers.user_id = $1
        "#,
        user_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get user's followers.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|user| AuthUser {
        id: user.id.to_string(),
        username: user.username,
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
    })
    .collect::<Vec<AuthUser>>();

    Ok(followers)
}

pub async fn get_following(conn: &PgPool, user_id: &Uuid) -> Result<Vec<AuthUser>> {
    let following = sqlx::query!(
        r#"
        SELECT users.id, users.username, users.email, users.first_name, users.last_name, users.description
        FROM users
        INNER JOIN users_followers ON users.id = users_followers.user_id
        WHERE users_followers.follower_id = $1
        "#,
        user_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get user's following.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|user| AuthUser {
        id: user.id.to_string(),
        username: user.username,
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
    })
    .collect::<Vec<AuthUser>>();

    Ok(following)
}

pub async fn unfollow_user(conn: &PgPool, user_id: &Uuid, followed_id: &Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM users_followers
        WHERE user_id = $1 AND follower_id = $2
        "#,
        followed_id,
        user_id
    )
    .execute(conn)
    .await
    .context("Failed to unfollow user.")
    .map_err(ApiError::Database)?;
    Ok(())
}

pub async fn get_all_users(conn: &PgPool) -> Result<Vec<AuthUser>> {
    let users = sqlx::query!(
        r#"
        SELECT id, username, email, first_name, last_name, description
        FROM users
        "#,
    )
    .fetch_all(conn)
    .await
    .context("Failed to get all users.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|user| AuthUser {
        id: user.id.to_string(),
        username: user.username,
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
    })
    .collect::<Vec<AuthUser>>();

    Ok(users)
}

pub async fn get_users_by_query(query: String, conn: &PgPool) -> Result<Vec<AuthUser>> {
    let users = sqlx::query!(
        r#"
        SELECT id, username, email, first_name, last_name, description
        FROM users
        WHERE username LIKE $1
        "#,
        format!("%{}%", query.to_lowercase())
    )
    .fetch_all(conn)
    .await
    .context("Failed to get users by query.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|user| AuthUser {
        id: user.id.to_string(),
        username: user.username,
        name: format!("{} {}", user.first_name, user.last_name),
        description: user.description,
        email: user.email,
    })
    .collect::<Vec<AuthUser>>();

    Ok(users)
}

pub async fn get_users_feed(conn: &PgPool, user_id: &Uuid) -> Result<Vec<Post>> {
    let posts = sqlx::query!(
        r#"
        SELECT posts.id, posts.title, posts.location, posts.content, posts.author, posts.created_at, 
        (SELECT COUNT(*) FROM comments WHERE comments.post_id = posts.id) AS "num_comments!", 
        (SELECT COUNT(*) FROM likes WHERE likes.post_id = posts.id) AS "num_likes!"
        FROM posts
        INNER JOIN users_followers ON posts.author = users_followers.user_id
        WHERE users_followers.follower_id = $1
        ORDER BY posts.created_at DESC
        "#,
        user_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get user's feed.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|post| Post {
        id: post.id.to_string(),
        title: post.title,
        location: post.location,
        content: post.content,
        author: post.author.to_string(),
        created_at: post.created_at.timestamp(),
        num_comments: post.num_comments as u32,
        num_likes: post.num_likes as u32,
    })
    .collect::<Vec<Post>>();

    Ok(posts)
}
