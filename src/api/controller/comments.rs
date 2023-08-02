use anyhow::anyhow;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::{
    database,
    models::{
        error::{ApiError, Result},
        Comment, CreateComment,
    },
};

pub async fn create_comment(
    user_id: &Uuid,
    post_id: &Uuid,
    comment: CreateComment,
    conn: &PgPool,
) -> Result<String> {
    // Check if user exists
    let user = database::get_user_by_id(conn, user_id).await?;
    if user.is_none() {
        return Err(ApiError::NotFound(anyhow!("User does not exist")));
    }
    // Check if posts exists
    let post = database::get_post_by_id(conn, post_id).await?;
    if post.is_none() {
        return Err(ApiError::NotFound(anyhow!("Post does not exist")));
    }
    // Create comment
    let comment_id = database::create_comment(comment, user_id, post_id, conn).await?;
    Ok(comment_id)
}

pub async fn get_comments(post_id: &Uuid, conn: &PgPool) -> Result<Vec<Comment>> {
    // Check if post exists
    let post = database::get_post_by_id(conn, post_id).await?;
    if post.is_none() {
        return Err(ApiError::NotFound(anyhow!("Post does not exist")));
    }
    // Get comments
    let comments = database::get_comments(post_id, conn).await?;
    Ok(comments)
}
