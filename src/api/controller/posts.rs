use anyhow::{anyhow, Context};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::{
    database,
    error::{ApiError, Result},
    CreatePost, Post,
};

pub async fn get_users_post(conn: &PgPool, user_id: String) -> Result<Vec<Post>> {
    let user_id = Uuid::parse_str(&user_id)
        .context("Failed to convert user id to UUID")
        .map_err(ApiError::BadRequest)?;
    // Check if the user exists
    let user = database::get_user_by_id(conn, &user_id).await?;
    if user.is_none() {
        return Err(ApiError::NotFound(anyhow::anyhow!("User does not exist")));
    }
    // Get all posts by the user
    let posts = database::get_users_posts(conn, &user_id).await?;
    Ok(posts)
}

pub async fn create_post(conn: &PgPool, user_id: Uuid, post: CreatePost) -> Result<String> {
    let post_id = database::insert_post(conn, user_id, post).await?;
    Ok(post_id)
}

pub async fn get_users_feed(conn: &PgPool, user_id: Uuid) -> Result<Vec<Post>> {
    // Check that the user exists
    let user = database::get_user_by_id(conn, &user_id).await?;
    if user.is_none() {
        return Err(ApiError::NotFound(anyhow!("User does not exist")));
    }

    // Get the users feed
    let feed = database::get_users_feed(conn, &user_id).await?;

    Ok(feed)
}
