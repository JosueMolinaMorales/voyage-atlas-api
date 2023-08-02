use crate::api::models::{
    error::{ApiError, Result},
    CreatePost, Post,
};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_users_posts(conn: &PgPool, user_id: &Uuid) -> Result<Vec<Post>> {
    let posts = sqlx::query!(
        r#"
        SELECT id, title, location, author, content, created_at,
        (SELECT COUNT(*) FROM comments WHERE comments.post_id = posts.id) AS "num_comments!", 
        (SELECT COUNT(*) FROM likes WHERE likes.post_id = posts.id) AS "num_likes!"
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
        num_likes: post.num_likes as u32,
        num_comments: post.num_comments as u32,
    })
    .collect::<Vec<Post>>();

    Ok(posts)
}

pub async fn insert_post(conn: &PgPool, user_id: Uuid, new_post: CreatePost) -> Result<String> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO posts (id, title, location, author, content)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        id,
        new_post.title,
        new_post.location,
        user_id,
        new_post.content
    )
    .execute(conn)
    .await
    .context("Failed to insert new post into database.")
    .map_err(ApiError::Database)?;
    Ok(id.to_string())
}

pub async fn get_post_by_id(conn: &PgPool, post_id: &Uuid) -> Result<Option<Post>> {
    let post = sqlx::query!(
        r#"
        SELECT id, title, location, author, content, created_at,
        (SELECT COUNT(*) FROM comments WHERE comments.post_id = posts.id) AS "num_comments!", 
        (SELECT COUNT(*) FROM likes WHERE likes.post_id = posts.id) AS "num_likes!"
        FROM posts
        WHERE id = $1
        "#,
        post_id
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get post by id.")
    .map_err(ApiError::Database)?
    .map(|post| Post {
        id: post.id.to_string(),
        title: post.title,
        location: post.location,
        content: post.content,
        author: post.author.to_string(),
        created_at: post.created_at.timestamp(),
        num_comments: post.num_comments as u32,
        num_likes: post.num_likes as u32,
    });

    Ok(post)
}
