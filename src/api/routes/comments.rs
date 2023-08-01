use crate::api::{
    controller,
    error::{ApiError, Result},
    token::JwtPayload,
    CreateComment,
};
use actix_web::{
    delete, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[post("/post/{post_id}/comment")]
async fn create_comment(
    post_id: Path<(String,)>,
    token: JwtPayload,
    comment: Json<CreateComment>,
    conn: Data<PgPool>,
) -> Result<HttpResponse> {
    let (post_id,) = post_id.into_inner();
    let post_id = Uuid::parse_str(&post_id)
        .context("Failed to parse post id")
        .map_err(ApiError::BadRequest)?;
    comment
        .validate()
        .context("Validation failed, comment should be greater than 1 and less than 255 characters")
        .map_err(ApiError::BadRequest)?;
    let user_id = Uuid::parse_str(&token.user_id)
        .context("Failed to convert user id from token")
        .map_err(ApiError::InternalServer)?;

    let comment_id =
        controller::comments::create_comment(&user_id, &post_id, comment.into_inner(), &conn)
            .await?;
    Ok(HttpResponse::Created().json(json!({ "comment_id": comment_id })))
}

#[post("/post/{post_id}/comment/{comment_id}/reply")]
async fn reply_to_comment() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[delete("/post/{post_id}/comment/{comment_id}")]
async fn delete_comment() -> HttpResponse {
    HttpResponse::Ok().finish()
}
