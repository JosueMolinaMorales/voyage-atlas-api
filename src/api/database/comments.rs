use crate::api::models::{
    error::{ApiError, Result},
    Comment, CreateComment,
};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_comment(
    new_comment: CreateComment,
    user_id: &Uuid,
    post_id: &Uuid,
    conn: &PgPool,
) -> Result<String> {
    let comment_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO comments (id, user_id, post_id, comment)
        VALUES ($1, $2, $3, $4)
        "#,
        &comment_id,
        user_id,
        post_id,
        new_comment.comment
    )
    .execute(conn)
    .await
    .context("Failed to insert new comment into database.")
    .map_err(ApiError::Database)?;
    Ok(comment_id.to_string())
}

pub async fn get_comments(post_id: &Uuid, conn: &PgPool) -> Result<Vec<Comment>> {
    let comments = sqlx::query!(
        r#"
            SELECT * from comments
            where post_id = $1
        "#,
        post_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get comments")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|row| Comment {
        id: row.id.to_string(),
        user_id: row.user_id.to_string(),
        post_id: row.post_id.to_string(),
        comment: row.comment,
        created_at: row.created_at.timestamp(),
    })
    .collect::<Vec<Comment>>();

    Ok(comments)
}
