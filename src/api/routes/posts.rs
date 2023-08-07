use crate::api::{
    controller,
    models::{
        error::{ApiError, Result},
        token::JwtPayload,
        CreatePost,
    },
};
use actix_web::{
    delete, get, post,
    web::{self, Data, Json, Path},
    HttpResponse,
};
use anyhow::{anyhow, Context};
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub fn init_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users_post)
        .service(create_post)
        .service(get_users_feed)
        .service(like_a_post)
        .service(unlike_a_post)
        .service(get_likes_of_post);
}

#[get("/users/{user_id}/posts")]
#[tracing::instrument(name = "Get A Users Post", skip(conn))]
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

#[get("/post/{post_id}/like")]
#[tracing::instrument(name = "Get Likes for a post", skip(path, conn))]
async fn get_likes_of_post(path: Path<(String,)>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let (post_id,) = path.into_inner();

    let post_id = Uuid::from_str(&post_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::BadRequest)?;

    let likes = controller::posts::get_likes_of_post(&post_id, &conn).await?;

    Ok(HttpResponse::Ok().json(json!(likes)))
}

#[post("/post/{post_id}/like")]
#[tracing::instrument(name = "Like a Post", skip(path, token, conn))]
async fn like_a_post(
    token: JwtPayload,
    path: Path<(String,)>,
    conn: Data<PgPool>,
) -> Result<HttpResponse> {
    let (post_id,) = path.into_inner();
    let user_id = Uuid::from_str(&token.user_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::InternalServer)?;
    let post_id = Uuid::from_str(&post_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::BadRequest)?;

    controller::posts::like_a_post(&user_id, &post_id, &conn).await?;

    Ok(HttpResponse::Created().finish())
}

#[delete("/post/{post_id}/like")]
#[tracing::instrument(name = "Unlike a Post", skip(path, token, conn))]
async fn unlike_a_post(
    token: JwtPayload,
    path: Path<(String,)>,
    conn: Data<PgPool>,
) -> Result<HttpResponse> {
    let (post_id,) = path.into_inner();
    let user_id = Uuid::from_str(&token.user_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::InternalServer)?;
    let post_id = Uuid::from_str(&post_id)
        .context("Failed to convert UUID")
        .map_err(ApiError::BadRequest)?;

    controller::posts::unlike_a_post(&user_id, &post_id, &conn).await?;

    Ok(HttpResponse::NoContent().finish())
}
