use std::str::FromStr;

use serde_json::{json, Value};
use uuid::Uuid;
use voyage_atlas_api::api::models::{Comment, CreateComment, Post};

use crate::helpers::spawn_app;

#[tokio::test]
async fn test_create_comment() {
    let test_app = spawn_app().await;
    // Create a post
    let res = test_app
        .create_post(
            json!({
                "title": "My first post",
                "location": "location",
                "content": "content"
            }),
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 201);
    let json = res.json::<Value>().await.unwrap();
    let post_id = json.get("post_id").unwrap().as_str().unwrap();

    // Create a comment
    let res = test_app
        .create_comment(
            post_id,
            CreateComment {
                comment: "My first comment".to_string(),
            },
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 201);
    let comment_id = Uuid::from_str(
        &res.json::<serde_json::Value>().await.unwrap()["comment_id"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    // Check that the comment was created
    let post = sqlx::query!(
        r#"
        SELECT * from comments
        WHERE id = $1
    "#,
        comment_id
    )
    .fetch_optional(&test_app.db_pool)
    .await
    .unwrap();

    assert!(post.is_some());

    // Get a post
    let res = test_app
        .get_user_posts(&test_app.auth_info.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 200);
    let post = res.json::<Vec<Post>>().await.unwrap();
    assert_eq!(post[0].num_comments, 1);
}

#[tokio::test]
async fn test_create_comment_fails_post_dne() {
    let test_app = spawn_app().await;
    // Create a comment
    let res = test_app
        .create_comment(
            &Uuid::new_v4().to_string(),
            CreateComment {
                comment: "Comment".into(),
            },
            &test_app.auth_info.bearer,
        )
        .await;

    assert_eq!(res.status().as_u16(), 404);
    // Check error message
    let json = res.json::<Value>().await.unwrap();
    assert_eq!(json["error"], "Post does not exist");
}

#[tokio::test]
async fn test_create_comment_fails_comment_too_short() {
    let test_app = spawn_app().await;
    // Create a post
    let res = test_app
        .create_post(
            json!({
                "title": "My first post",
                "location": "location",
                "content": "content"
            }),
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 201);
    let json = res.json::<Value>().await.unwrap();
    let post_id = json.get("post_id").unwrap().as_str().unwrap();

    // Create comment
    let res = test_app
        .create_comment(
            post_id,
            CreateComment { comment: "".into() },
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn test_create_comment_fails_comment_too_long() {
    let test_app = spawn_app().await;
    // Create a post
    let res = test_app
        .create_post(
            json!({
                "title": "My first post",
                "location": "location",
                "content": "content"
            }),
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 201);
    let json = res.json::<Value>().await.unwrap();
    let post_id = json.get("post_id").unwrap().as_str().unwrap();

    // Create comment
    let res = test_app
        .create_comment(
            post_id,
            CreateComment {
                comment: "a".repeat(256),
            },
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn test_get_comments_on_post() {
    let test_app = spawn_app().await;
    // Create a post
    let res = test_app
        .create_post(
            json!({
                "title": "My first post",
                "location": "location",
                "content": "content"
            }),
            &test_app.auth_info.bearer,
        )
        .await;
    assert_eq!(res.status().as_u16(), 201);
    let json = res.json::<Value>().await.unwrap();
    let post_id = json.get("post_id").unwrap().as_str().unwrap();

    // Create comments
    for i in 0..5 {
        let res = test_app
            .create_comment(
                post_id,
                CreateComment {
                    comment: format!("Comment {}", i),
                },
                &test_app.auth_info.bearer,
            )
            .await;
        assert_eq!(res.status().as_u16(), 201);
    }

    // Get comments
    let res = test_app.get_comments(post_id).await;
    assert_eq!(res.status().as_u16(), 200);
    let comments = res.json::<Vec<Comment>>().await.unwrap();
    assert_eq!(comments.len(), 5);
}
