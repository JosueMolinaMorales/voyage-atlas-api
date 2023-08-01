use crate::api::{
    database,
    error::{ApiError, Result},
    CreateComment,
};
use anyhow::anyhow;
use sqlx::PgPool;
use uuid::Uuid;

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
