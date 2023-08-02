#[derive(serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub location: String,
    pub content: String,
    pub author: String,
    pub created_at: i64,
    pub num_likes: u32,
    pub num_comments: u32,
}

#[derive(serde::Deserialize, validator::Validate)]
pub struct CreatePost {
    #[validate(length(min = 3), length(max = 20))]
    pub title: String,
    #[validate(length(min = 3), length(max = 100))]
    pub location: String,
    #[validate(length(min = 3), length(max = 255))]
    pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Like {
    pub user_id: String,
    pub post_id: String,
    pub created_at: i64,
}
