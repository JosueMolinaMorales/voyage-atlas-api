/*
    TODO: Write tests for
    - Creating a post
    - Gettting a post
*/

use std::str::FromStr;

use uuid::Uuid;
use voyage_atlas_api::api::models::{Like, Post};

use crate::helpers::{spawn_app, TestAuthInfo};

#[tokio::test]
async fn test_creating_a_post() {
    let test_app = spawn_app().await;
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);
    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Check that the post was created
    let post = sqlx::query!(
        r#"
        SELECT * from posts
        WHERE id = $1"#,
        post_id
    )
    .fetch_optional(&test_app.db_pool)
    .await
    .unwrap();

    assert!(post.is_some());
}

#[tokio::test]
async fn test_get_user_posts() {
    let test_app = spawn_app().await;
    // Create posts
    for i in 0..5 {
        let body = serde_json::json!({
            "title": format!("My first post {}", i),
            "location": "location",
            "content": "content"
        });
        let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
        assert_eq!(response.status().as_u16(), 201);
    }
    // Get the post
    let res = test_app
        .get_user_posts(&test_app.auth_info.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 200);
    let posts: Vec<Post> = res.json().await.unwrap();
    assert_eq!(posts.len(), 5);
    // Check that the posts have 0 likes and comments
    for post in posts {
        assert_eq!(post.num_likes, 0);
        assert_eq!(post.num_comments, 0);
    }
}

#[tokio::test]
async fn test_get_user_posts_user_dne() {
    let test_app = spawn_app().await;
    let res = test_app
        .get_user_posts(&Uuid::new_v4().to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 404);
    // Check error message
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.get("error").unwrap(), "User does not exist");
}

#[tokio::test]
async fn test_get_user_posts_user_id_invalid() {
    let test_app = spawn_app().await;
    let res = test_app
        .get_user_posts("dasfds", &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn test_get_a_users_feed() {
    // Setup
    let test_app = spawn_app().await;
    // Follow a user
    let user_to_follow = TestAuthInfo::generate();
    user_to_follow.store(&test_app.db_pool).await;

    let res = test_app
        .follow_user(&user_to_follow.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Create posts
    for i in 0..10 {
        let body = serde_json::json!({
            "title": format!("My first post {}", i),
            "location": "location",
            "content": "content"
        });
        let response = test_app.create_post(body, &user_to_follow.bearer).await;
        assert_eq!(response.status().as_u16(), 201);
    }
    // Get the feed
    let res = test_app.get_user_feed(&test_app.auth_info.bearer).await;
    assert_eq!(res.status().as_u16(), 200);
    let posts: Vec<Post> = res.json().await.unwrap();
    assert_eq!(posts.len(), 10);
    // Assert that the posts are sorted in descending order
    let mut prev_post = &posts[0];
    for post in &posts[1..] {
        assert!(post.created_at <= prev_post.created_at);
        prev_post = post;
    }
}

#[tokio::test]
async fn test_get_a_users_feed_not_following_anyone() {
    // Setup
    let test_app = spawn_app().await;
    // Get the feed
    let res = test_app.get_user_feed(&test_app.auth_info.bearer).await;
    assert_eq!(res.status().as_u16(), 200);
    let posts: Vec<Post> = res.json().await.unwrap();
    assert_eq!(posts.len(), 0);
}

#[tokio::test]
async fn test_liking_a_post() {
    let test_app = spawn_app().await;
    // Create a post
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);
    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Like the post
    let res = test_app
        .like_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Check that the post was liked
    let like = sqlx::query!(
        r#"
        SELECT * from likes
        WHERE post_id = $1 AND user_id = $2"#,
        post_id,
        Uuid::from_str(&test_app.auth_info.user.id).unwrap()
    )
    .fetch_optional(&test_app.db_pool)
    .await
    .unwrap();

    assert!(like.is_some());
}

#[tokio::test]
async fn test_liking_a_post_fail_post_dne() {
    let test_app = spawn_app().await;
    // Like the post
    let res = test_app
        .like_a_post(&Uuid::new_v4().to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 404);
    // Check error message
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.get("error").unwrap(), "Post does not exist");
}

#[tokio::test]
async fn test_liking_a_post_fail_already_liked() {
    let test_app = spawn_app().await;
    // Create a post
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);

    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Like the post
    let res = test_app
        .like_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Like the post again
    let res = test_app
        .like_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 400);
    // Check error message
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(
        json.get("error").unwrap().as_str().unwrap(),
        "You have already liked this post"
    );
}

#[tokio::test]
async fn test_unliking_a_post() {
    let test_app = spawn_app().await;
    // Create a post
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);

    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Like the post
    let res = test_app
        .like_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Unlike the post
    let res = test_app
        .unlike_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 204);
    // Check that the post was unliked
    let like = sqlx::query!(
        r#"
        SELECT * from likes
        WHERE post_id = $1 AND user_id = $2"#,
        post_id,
        Uuid::from_str(&test_app.auth_info.user.id).unwrap()
    )
    .fetch_optional(&test_app.db_pool)
    .await
    .unwrap();

    assert!(like.is_none());
}

#[tokio::test]
async fn test_unliking_a_post_fail_post_dne() {
    let test_app = spawn_app().await;
    // Unlike the post
    let res = test_app
        .unlike_a_post(&Uuid::new_v4().to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 404);
    // Check error message
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.get("error").unwrap(), "Post not found");
}

#[tokio::test]
async fn test_unliking_a_post_fail_not_liked() {
    let test_app = spawn_app().await;
    // Create a post
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);

    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Unlike the post
    let res = test_app
        .unlike_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 400);
    // Check error message
    let json = res.json::<serde_json::Value>().await.unwrap();
    assert_eq!(json.get("error").unwrap(), "Post not liked");
}

#[tokio::test]
async fn test_get_likes_for_a_post() {
    let test_app = spawn_app().await;
    // Create a post
    let body = serde_json::json!({
        "title": "My first post",
        "location": "location",
        "content": "content"
    });
    let response = test_app.create_post(body, &test_app.auth_info.bearer).await;
    assert_eq!(response.status().as_u16(), 201);

    let json = response.json::<serde_json::Value>().await.unwrap();
    let post_id = Uuid::from_str(json.get("post_id").unwrap().as_str().unwrap()).unwrap();
    // Like the post
    let res = test_app
        .like_a_post(&post_id.to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Get likes for the post
    let res = test_app.get_likes_for_a_post(&post_id.to_string()).await;
    assert_eq!(res.status().as_u16(), 200);
    // Check that the post was liked
    let likes = res.json::<Vec<Like>>().await.unwrap();
    assert_eq!(likes.len(), 1);
    assert_eq!(likes[0].user.id, test_app.auth_info.user.id);
}
