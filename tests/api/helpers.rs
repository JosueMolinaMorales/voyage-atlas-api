use once_cell::sync::Lazy;
use serde_json::json;
use sqlx::{sqlx_macros::migrate, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use voyage_atlas_api::api::{
    get_configuration, get_connection_pool, get_subscriber, init_subscriber, Application, AuthInfo,
    AuthUser, DatabaseSettings,
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
    pub auth_info: AuthInfo,
}

impl TestApp {
    pub async fn post_user(&self, body: serde_json::Value) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users", &self.address);
        client.post(&url).json(&body).send().await.unwrap()
    }

    pub async fn create_post(&self, body: serde_json::Value, bearer: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/post", &self.address);
        client
            .post(&url)
            .bearer_auth(bearer)
            .json(&body)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_user_posts(&self, user_id: &str, bearer: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/posts", &self.address, user_id);
        client.get(&url).bearer_auth(bearer).send().await.unwrap()
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failted to read configuration");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // use a random OS port
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    // Launch the app
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to create app");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    let mut test_app = TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        auth_info: AuthInfo {
            bearer: "".to_string(),
            user: AuthUser {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
            },
        },
    };

    // Create a user
    let auth_info = {
        let res = test_app
            .post_user(json!({
                "email": "email@email.com",
                "password": "Password123!",
                "username": "username"
            }))
            .await;
        let user = res.json::<AuthInfo>().await.unwrap();
        user
    };
    test_app.auth_info = auth_info;
    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connection to postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate Database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to postgres");
    migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
