use anyhow::{anyhow, Context};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::{
    database,
    models::{
        error::{ApiError, Result},
        CreatePost, Like, Post,
    },
};

pub async fn get_likes_of_post(post_id: &Uuid, conn: &PgPool) -> Result<Vec<Like>> {
    // Check that the post exists
    let post = database::get_post_by_id(conn, post_id).await?;
    if post.is_none() {
        return Err(ApiError::NotFound(anyhow!("Post does not exist")));
    }
    // Get the likes of the post
    let like = database::get_likes_of_post(conn, post_id).await?;
    Ok(like)
}

#[tracing::instrument("Controller: Get a users posts", skip(conn))]
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

pub async fn like_a_post(user_id: &Uuid, post_id: &Uuid, conn: &PgPool) -> Result<()> {
    // Check that the user exists
    let user = database::get_user_by_id(conn, user_id).await?;
    if user.is_none() {
        return Err(ApiError::NotFound(anyhow!("User does not exist")));
    }
    // Check that the post exists
    let post = database::get_post_by_id(conn, post_id).await?;
    if post.is_none() {
        return Err(ApiError::NotFound(anyhow!("Post does not exist")));
    }
    // Check that the user has not already liked the post
    let like = database::get_like_by_user_and_post(conn, user_id, post_id).await?;
    if like.is_some() {
        return Err(ApiError::BadRequest(anyhow!(
            "You have already liked this post"
        )));
    }
    // Like the post
    database::like_post(conn, user_id, post_id).await?;

    Ok(())
}

pub async fn unlike_a_post(user_id: &Uuid, post_id: &Uuid, conn: &PgPool) -> Result<()> {
    // Check that the user exists
    let user = database::get_user_by_id(conn, user_id).await?;
    if user.is_none() {
        return Err(ApiError::NotFound(anyhow!("User does not exist")));
    }
    // Check that the post exists
    let post = database::get_post_by_id(conn, post_id).await?;
    if post.is_none() {
        return Err(ApiError::NotFound(anyhow!("Post not found")));
    }
    // Check that the user has liked the post
    let like = database::get_like_by_user_and_post(conn, user_id, post_id).await?;
    if like.is_none() {
        return Err(ApiError::BadRequest(anyhow!("Post not liked")));
    }
    // Unlike the post
    database::unlike_post(conn, user_id, post_id).await?;

    Ok(())
}
