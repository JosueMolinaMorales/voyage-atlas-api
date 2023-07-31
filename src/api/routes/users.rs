use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::api::{
    controller,
    error::{ApiError, Result},
    token::JwtPayload,
    CreateUser, LoginInfo,
};

#[post("/users")]
#[tracing::instrument(name = "Create a new user", skip(new_user, conn))]
async fn create_user(new_user: Json<CreateUser>, conn: Data<PgPool>) -> Result<HttpResponse> {
    // Validate new user
    new_user.0.validate().map_err(|err| {
        // TODO: Return a more specific error
        let errors = err
            .errors()
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        ApiError::BadRequest(anyhow::anyhow!("Invalid fields: {}", errors))
    })?;
    let auth_info = controller::user::register(new_user.0, &conn).await?;
    Ok(HttpResponse::Created().json(auth_info))
}

#[post("/users/login")]
#[tracing::instrument(name = "Logging a user in", skip(login_info, conn))]
async fn login(login_info: Json<LoginInfo>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let auth_info = controller::user::login(login_info.0, &conn).await?;
    Ok(HttpResponse::Ok().json(auth_info))
}

#[post("/users/{user_id}/follow")]
#[tracing::instrument(name = "Follow a user", skip(conn))]
async fn follow_user(
    token: JwtPayload,
    followed_user: Path<(String,)>,
    conn: Data<PgPool>,
) -> Result<HttpResponse> {
    let (followed_user_id,) = followed_user.into_inner();
    let followed_user_id =
        Uuid::parse_str(&followed_user_id).map_err(|e| ApiError::BadRequest(anyhow::anyhow!(e)))?;
    let user_id =
        Uuid::parse_str(&token.user_id).map_err(|e| ApiError::BadRequest(anyhow::anyhow!(e)))?;

    controller::user::follow_user(user_id, followed_user_id, &conn).await?;

    Ok(HttpResponse::Created().finish())
}
