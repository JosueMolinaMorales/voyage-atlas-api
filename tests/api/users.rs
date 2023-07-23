use serde_json::{json, Value};

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_user() {
    let test_app = spawn_app().await;
    let res = test_app
        .post_user(json!({
            "username": "testuser",
            "password": "Password123!",
            "email": "email@email.com"
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
    // Create user
    test_app
        .post_user(json!({
            "username": "testuser",
            "password": "Password123!",
            "email": "email@email.com"
        }))
        .await;

    let res = reqwest::Client::new()
        .post(format!("{}/users/login", test_app.address))
        .json(&json!({
            "email": "email@email.com",
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
    // Create user
    test_app
        .post_user(json!({
            "username": "testuser",
            "password": "Password123!",
            "email": "email@email.com",
        }))
        .await;
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
                "email": "email@email.com",
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
