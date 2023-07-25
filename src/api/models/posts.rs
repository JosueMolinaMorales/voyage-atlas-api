#[derive(serde::Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub location: String,
    pub content: String,
    pub author: String,
    pub created_at: i64,
}
