use crate::api::{
    controller,
    error::{ApiError, Result},
    token::JwtPayload,
    CreatePost,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::{anyhow, Context};
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

#[get("/users/{user_id}/posts")]
#[tracing::instrument(name = "Get A Users Post", skip(path, conn))]
async fn get_users_post(path: Path<(String,)>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let (user_id,) = path.into_inner();
    let posts = controller::posts::get_users_post(&conn, user_id).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/users/post")]
#[tracing::instrument(name = "Create a New Post", skip(new_post, jwt, conn))]
async fn create_post(
    new_post: Json<CreatePost>,
    jwt: JwtPayload,
    conn: Data<PgPool>,
) -> Result<HttpResponse> {
    let user_id =
        uuid::Uuid::from_str(&jwt.user_id).map_err(|e| ApiError::InternalServer(anyhow!(e)))?;

    let post_id = controller::posts::create_post(&conn, user_id, new_post.0).await?;

    Ok(HttpResponse::Created().json(json!({ "post_id": post_id })))
}

#[get("/feed")]
#[tracing::instrument(name = "Get A Users Feed", skip(token, conn))]
async fn get_users_feed(token: JwtPayload, conn: Data<PgPool>) -> Result<HttpResponse> {
    // TODO: Implement
    let user_id = Uuid::from_str(&token.user_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::BadRequest)?;
    /*
       A users feed will be a list of posts from users that the user follows sorted by date.
    */
    let feed = controller::posts::get_users_feed(&conn, user_id).await?;
    Ok(HttpResponse::Ok().json(feed))
}
