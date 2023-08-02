use once_cell::sync::Lazy;
use sqlx::{sqlx_macros::migrate, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use voyage_atlas_api::api::{
    configuration::{get_configuration, DatabaseSettings},
    models::{token, AuthUser, CreateComment},
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber, init_subscriber},
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
    pub auth_info: TestAuthInfo,
}

#[derive(Debug)]
pub struct TestAuthInfo {
    pub bearer: String,
    pub user: AuthUser,
}

impl TestAuthInfo {
    pub fn new(username: &str) -> Self {
        let id = Uuid::new_v4().to_string();
        let token = token::generate_token(&id).unwrap();
        TestAuthInfo {
            bearer: token,
            user: AuthUser {
                id,
                username: username.to_string(),
                email: format!("{}@email", username),
            },
        }
    }

    pub fn generate() -> Self {
        let id = Uuid::new_v4().to_string();
        let token = token::generate_token(&id).unwrap();
        TestAuthInfo {
            bearer: token,
            user: AuthUser {
                id,
                username: Uuid::new_v4().to_string(),
                email: format!("{}@email", Uuid::new_v4().to_string()),
            },
        }
    }

    pub async fn store(&self, pool: &PgPool) {
        let password = pwhash::bcrypt::hash("Password123!").unwrap();
        let id = Uuid::parse_str(&self.user.id).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            self.user.username,
            self.user.email,
            password
        )
        .execute(pool)
        .await
        .unwrap();
    }
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

    pub async fn follow_user(&self, followed_user: &str, bearer: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/follow", &self.address, followed_user);
        client.post(&url).bearer_auth(bearer).send().await.unwrap()
    }

    pub async fn get_followers(&self, user_id: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/followers", &self.address, user_id);
        client.get(&url).send().await.unwrap()
    }

    pub async fn get_following(&self, user_id: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/following", &self.address, user_id);
        client.get(&url).send().await.unwrap()
    }

    pub async fn unfollow_user(&self, followed_user: &str, bearer: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/users/{}/unfollow", &self.address, followed_user);
        client
            .delete(&url)
            .bearer_auth(bearer)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_all_users(&self, query: Option<String>) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = match query {
            Some(q) => format!("{}/users?query={}", &self.address, q),
            None => format!("{}/users", &self.address),
        };
        client.get(&url).send().await.unwrap()
    }

    pub async fn get_user_feed(&self, token: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/feed", &self.address,);
        client.get(&url).bearer_auth(token).send().await.unwrap()
    }

    pub async fn create_comment(
        &self,
        post_id: &str,
        comment: CreateComment,
        token: &str,
    ) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/post/{}/comment", &self.address, post_id);
        client
            .post(&url)
            .bearer_auth(token)
            .json(&comment)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_comments(&self, post_id: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        let url = format!("{}/post/{}/comment", &self.address, post_id);
        client.get(&url).send().await.unwrap()
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

    let test_app = TestApp {
        address,
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        auth_info: TestAuthInfo::generate(),
    };

    // Create a user
    test_app.auth_info.store(&test_app.db_pool).await;

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
