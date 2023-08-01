use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1), length(max = 255))]
    pub comment: String,
}
