use crate::api::models::{
    error::{ApiError, Result},
    AuthUser, CreatePost, Like, Post,
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

pub async fn get_like_by_user_and_post(
    conn: &PgPool,
    user_id: &Uuid,
    post_id: &Uuid,
) -> Result<Option<Like>> {
    let like = sqlx::query!(
        r#"
        SELECT user_id, post_id, likes.created_at, username, email
        FROM likes, users
        WHERE likes.user_id = users.id AND likes.user_id = $1 AND likes.post_id = $2
        "#,
        user_id,
        post_id
    )
    .fetch_optional(conn)
    .await
    .context("Failed to get like by user and post.")
    .map_err(ApiError::Database)?
    .map(|like| Like {
        post_id: like.post_id.to_string(),
        created_at: like.created_at.timestamp(),
        user: AuthUser {
            id: like.user_id.to_string(),
            username: like.username,
            email: like.email,
        },
    });

    Ok(like)
}

pub async fn get_likes_of_post(conn: &PgPool, post_id: &Uuid) -> Result<Vec<Like>> {
    let likes = sqlx::query!(
        r#"
        SELECT user_id, post_id, likes.created_at, username, email
        FROM likes, users
        WHERE likes.user_id = users.id AND likes.post_id = $1
        "#,
        post_id
    )
    .fetch_all(conn)
    .await
    .context("Failed to get likes of post.")
    .map_err(ApiError::Database)?
    .into_iter()
    .map(|like| Like {
        user: AuthUser {
            id: like.user_id.to_string(),
            username: like.username,
            email: like.email,
        },
        post_id: like.post_id.to_string(),
        created_at: like.created_at.timestamp(),
    })
    .collect::<Vec<Like>>();

    Ok(likes)
}

pub async fn like_post(conn: &PgPool, user_id: &Uuid, post_id: &Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO likes (user_id, post_id)
        VALUES ($1, $2)
        "#,
        user_id,
        post_id
    )
    .execute(conn)
    .await
    .context("Failed to like post.")
    .map_err(ApiError::Database)?;

    Ok(())
}

pub async fn unlike_post(conn: &PgPool, user_id: &Uuid, post_id: &Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM likes
        WHERE user_id = $1 AND post_id = $2
        "#,
        user_id,
        post_id
    )
    .execute(conn)
    .await
    .context("Failed to unlike post.")
    .map_err(ApiError::Database)?;

    Ok(())
}
