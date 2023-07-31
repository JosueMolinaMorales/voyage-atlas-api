use serde_json::{json, Value};

use crate::helpers::{spawn_app, TestAuthInfo};

#[tokio::test]
async fn create_user() {
    let test_app = spawn_app().await;
    let res = test_app
        .post_user(json!({
            "username": "testuser",
            "password": "Password123!",
            "email": "email123@email.com"
        }))
        .await;

    assert_eq!(res.status().as_u16(), 201);
    let token = res.json::<Value>().await.expect("failed to parse response");
    assert!(token.get("bearer").is_some());
}

#[tokio::test]
async fn create_user_fails_invalid_data() {
    let test_app = spawn_app().await;

    let test_data = vec![
        (
            json!({
                "username": "testuser",
                "password": "Password123!",
                "email": "email"
            }),
            vec!["email"],
        ),
        (
            json!({
                "username": "testuser",
                "password": "Password123",
                "email": "email"
            }),
            vec!["email", "password"],
        ),
        (
            json!({
                "username": "testuser",
                "password": "Password123",
                "email": "email@email.com"
            }),
            vec!["password"],
        ),
    ];

    for (data, expected_err) in test_data {
        let res = test_app.post_user(json!(data)).await;

        assert_eq!(res.status().as_u16(), 400);
        let body = res.json::<Value>().await.expect("failed to parse response");
        let error = body
            .get("error")
            .expect("failed to get error")
            .as_str()
            .expect("failed to get error as string");
        assert!(
            expected_err.iter().all(|e| error.contains(e)),
            "{}",
            format!(
                "expected error to contain {:?}, got {:?}",
                expected_err, error
            )
        );
    }
}

#[tokio::test]
async fn test_login() {
    let test_app = spawn_app().await;
    let res = reqwest::Client::new()
        .post(format!("{}/users/login", test_app.address))
        .json(&json!({
            "email": test_app.auth_info.user.email,
            "password": "Password123!"
        }))
        .send()
        .await
        .expect("failed to send request");

    let token = res.json::<Value>().await.expect("failed to parse response");
    assert!(token.get("bearer").is_some());
}

#[tokio::test]
async fn test_login_fails() {
    let test_app = spawn_app().await;

    let test_data = vec![
        (
            json!({
                "email": "email@123.com",
                "password": "Password123!"
            }),
            "Email was wrong",
        ),
        (
            json!({
                "email": test_app.auth_info.user.email,
                "password": "password"
            }),
            "Password was wrong",
        ),
    ];
    for (data, reason) in test_data {
        let res = reqwest::Client::new()
            .post(format!("{}/users/login", test_app.address))
            .json(&data)
            .send()
            .await
            .expect("failed to send request");

        assert_eq!(res.status().as_u16(), 400);
        let body = res.json::<Value>().await.expect("failed to parse response");
        let error = body
            .get("error")
            .expect("failed to get error")
            .as_str()
            .expect("failed to get error as string");
        assert!(
            error == "Email or Password is incorrect",
            "Login Validation Failed for: {reason}",
        );
    }
}

#[tokio::test]
async fn test_follow_user() {
    let test_app = spawn_app().await;
    // Create user
    let new_user = TestAuthInfo::generate();
    new_user.store(&test_app.db_pool).await;

    let res = test_app
        .follow_user(&new_user.user.id, &test_app.auth_info.bearer)
        .await;

    assert_eq!(res.status().as_u16(), 201);
}

#[tokio::test]
async fn test_follow_user_already_following_user() {
    let test_app = spawn_app().await;
    // Create user
    let new_user = TestAuthInfo::generate();
    new_user.store(&test_app.db_pool).await;

    // Follow user
    let res = test_app
        .follow_user(&new_user.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);
    // Follow user again
    let res = test_app
        .follow_user(&new_user.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 400);
}

#[tokio::test]
async fn test_get_users_followers() {
    let test_app = spawn_app().await;
    // Create user
    let new_user = TestAuthInfo::generate();
    new_user.store(&test_app.db_pool).await;

    // Follow user
    let res = test_app
        .follow_user(&new_user.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);

    // Get followers
    let res = test_app.get_followers(&new_user.user.id).await;
    assert_eq!(res.status().as_u16(), 200);
    let followers = res.json::<Vec<Value>>().await.unwrap();
    assert_eq!(followers.len(), 1);
}

#[tokio::test]
async fn test_get_users_following() {
    let test_app = spawn_app().await;
    // Create user
    let new_user = TestAuthInfo::generate();
    new_user.store(&test_app.db_pool).await;

    // Follow user
    let res = test_app
        .follow_user(&new_user.user.id, &test_app.auth_info.bearer)
        .await;
    assert_eq!(res.status().as_u16(), 201);

    // Get following
    let res = test_app.get_following(&test_app.auth_info.user.id).await;
    assert_eq!(res.status().as_u16(), 200);
    let following = res.json::<Vec<Value>>().await.unwrap();
    assert_eq!(following.len(), 1);
}
