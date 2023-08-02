use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1), length(max = 255))]
    pub comment: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub post_id: String,
    pub comment: String,
    pub created_at: i64,
}
