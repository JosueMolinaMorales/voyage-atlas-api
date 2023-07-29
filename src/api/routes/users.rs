use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use sqlx::PgPool;
use validator::Validate;

use crate::api::{
    controller,
    error::{ApiError, Result},
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
