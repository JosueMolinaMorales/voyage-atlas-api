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
use anyhow::anyhow;
use sqlx::PgPool;
use std::str::FromStr;

#[get("/users/{user_id}/posts")]
async fn get_users_post(path: Path<(String,)>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let (user_id,) = path.into_inner();
    let posts = controller::posts::get_users_post(&conn, user_id).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[post("/users/post")]
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
