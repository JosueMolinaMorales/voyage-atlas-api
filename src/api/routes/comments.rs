use actix_web::{
    delete, get, post,
    web::{self, Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::api::{
    controller,
    models::{
        error::{ApiError, Result},
        token::JwtPayload,
        CreateComment,
    },
};

pub fn init_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_comments)
        .service(create_comment)
        .service(delete_comment);
}

#[get("/post/{post_id}/comment")]
#[tracing::instrument(name = "Get Comments", skip(path, conn))]
async fn get_comments(path: Path<(String,)>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let (post_id,) = path.into_inner();
    let post_id = Uuid::parse_str(&post_id)
        .context("Failed to parse post id")
        .map_err(ApiError::BadRequest)?;
    let comments = controller::comments::get_comments(&post_id, &conn).await?;
    Ok(HttpResponse::Ok().json(comments))
}

#[post("/post/{post_id}/comment")]
#[tracing::instrument(name = "Create Comment", skip(post_id, token, comment, conn))]
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
#[tracing::instrument(name = "Reply to Comment")]
async fn reply_to_comment() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[delete("/post/{post_id}/comment/{comment_id}")]
#[tracing::instrument(name = "Delete Comment")]
async fn delete_comment() -> HttpResponse {
    HttpResponse::Ok().finish()
}
