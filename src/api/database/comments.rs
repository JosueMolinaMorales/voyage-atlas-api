use crate::api::models::{
    error::{ApiError, Result},
    AuthUser, Comment, CreateComment,
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
            SELECT comments.id, user_id, post_id, parent_comment_id, comments.created_at, comment,
            users.username, users.email as "user_email!", users.description, users.first_name, users.last_name
            FROM comments, users
            where comments.user_id = users.id and post_id = $1
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
        post_id: row.post_id.to_string(),
        comment: row.comment,
        created_at: row.created_at.timestamp(),
        parent_comment_id: row.parent_comment_id.map(|id| id.to_string()),
        user: AuthUser {
            id: row.user_id.to_string(),
            username: row.username,
            email: row.user_email,
            name: format!("{} {}", row.first_name, row.last_name),
            description: row.description
        },
    })
    .collect::<Vec<Comment>>();

    Ok(comments)
}

pub async fn get_comment_by_id(comment_id: &Uuid, conn: &PgPool) -> Result<Option<Comment>> {
    let comment = sqlx::query!(
        r#"
            SELECT comments.id, user_id, post_id, parent_comment_id, comments.created_at, comment,
            users.username, users.email as "user_email!", users.description, users.first_name, users.last_name
            FROM comments, users
            where comments.id = $1
        "#,
        comment_id
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get comment")
    .map_err(ApiError::Database)?
    .map(|row| Comment {
        id: row.id.to_string(),
        post_id: row.post_id.to_string(),
        comment: row.comment,
        created_at: row.created_at.timestamp(),
        parent_comment_id: row.parent_comment_id.map(|id| id.to_string()),
        user: AuthUser {
            id: row.user_id.to_string(),
            username: row.username,
            email: row.user_email,
            name: format!("{} {}", row.first_name, row.last_name),
            description: row.description
        },
    });

    Ok(comment)
}

pub async fn delete_comment(comment_id: &Uuid, conn: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM comments
        WHERE id = $1
        "#,
        comment_id
    )
    .execute(conn)
    .await
    .context("Failed to delete comment")
    .map_err(ApiError::Database)?;
    Ok(())
}

pub async fn reply_to_comment(
    comment: CreateComment,
    user_id: &Uuid,
    post_id: &Uuid,
    comment_id: &Uuid,
    conn: &PgPool,
) -> Result<()> {
    let reply_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO comments (id, user_id, post_id, comment, parent_comment_id)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        &reply_id,
        user_id,
        post_id,
        comment.comment,
        comment_id
    )
    .execute(conn)
    .await
    .context("Failed to insert new comment into database.")
    .map_err(ApiError::Database)?;
    Ok(())
}
