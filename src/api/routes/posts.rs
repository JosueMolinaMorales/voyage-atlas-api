use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use sqlx::PgPool;

use crate::api::{controller, error::Result};

#[get("/users/{user_id}/posts")]
async fn get_users_post(path: Path<(String,)>, conn: Data<PgPool>) -> Result<HttpResponse> {
    let (user_id,) = path.into_inner();
    let posts = controller::posts::get_users_post(&conn, user_id).await?;
    Ok(HttpResponse::Ok().json(posts))
}
