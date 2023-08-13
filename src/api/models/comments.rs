use validator::Validate;

use super::AuthUser;

#[derive(serde::Deserialize, serde::Serialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1), length(max = 255))]
    pub comment: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comment {
    pub id: String,
    pub user: AuthUser,
    pub post_id: String,
    pub comment: String,
    pub created_at: i64,
    pub parent_comment_id: Option<String>,
}
