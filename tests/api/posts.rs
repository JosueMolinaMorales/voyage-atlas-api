/*
    TODO: Write tests for
    - Creating a post
    - Gettting a post
*/

use uuid::Uuid;
use voyage_atlas_api::api::Post;

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
}

#[tokio::test]
async fn test_get_user_posts_user_dne() {
    let test_app = spawn_app().await;
    let res = test_app
        .get_user_posts(&Uuid::new_v4().to_string(), &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 404);
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
