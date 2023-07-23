use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::api::{CreateUser, error::{Result, ApiError}, controller, LoginInfo};

#[post("/users")]
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
    let token = controller::user::register(new_user.0, &conn).await?;
    Ok(HttpResponse::Created().json(json!({
        "bearer": token,
    })))
}

#[post("/users/login")]
async fn login(login_info: Json<LoginInfo>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let token = controller::user::login(login_info.0, &conn).await?;
    Ok(HttpResponse::Ok().json(json!({
        "bearer": token,
    })))
}