use crate::helpers::spawn_app;

#[tokio::test]
async fn test_health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new()
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(client.status().as_u16(), 200);
}
