use crate::api::{
    controller,
    error::{ApiError, Result},
    token::JwtPayload,
    CreatePost,
};
use actix_web::{
    get, post,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use anyhow::anyhow;
use sqlx::PgPool;
use std::str::FromStr;

#[derive(serde::Deserialize)]
struct UserSearchQuery {
    query: Option<String>,
}

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

    controller::posts::create_post(&conn, user_id, new_post.0).await?;

    Ok(HttpResponse::Created().finish())
}

#[get("/users/{user_id}/feed")]
#[tracing::instrument(name = "Get A Users Feed", skip(path, _conn))]
async fn get_users_feed(path: Path<(String,)>, _conn: Data<PgPool>) -> Result<HttpResponse> {
    // TODO: Implement
    let (_user_id,) = path.into_inner();
    Ok(HttpResponse::Ok().finish())
}

#[get("/users")]
#[tracing::instrument(name = "Get All Users", skip(conn, query))]
async fn get_all_users(query: Query<UserSearchQuery>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let users = controller::user::get_users(query.query.clone(), &conn).await?;
    Ok(HttpResponse::Ok().json(users))
}
