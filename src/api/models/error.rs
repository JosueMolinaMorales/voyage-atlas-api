use actix_web::ResponseError;
use serde_json::json;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(anyhow::Error),
    Unauthorized(anyhow::Error),
    Forbidden(anyhow::Error),
    Database(anyhow::Error),
    NotFound(anyhow::Error),
    InternalServer(anyhow::Error),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(message) => write!(f, "{message}",),
            ApiError::Unauthorized(message) => write!(f, "{message}",),
            ApiError::Forbidden(message) => write!(f, "{message}",),
            ApiError::NotFound(message) => write!(f, "{message}",),
            ApiError::Database(message) => write!(f, "Database Error: {message}",),
            ApiError::InternalServer(message) => {
                write!(f, "Internal Service Error: {message}",)
            }
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ApiError::BadRequest(_) => reqwest::StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => reqwest::StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => reqwest::StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => reqwest::StatusCode::NOT_FOUND,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut response_builder = actix_web::HttpResponse::build(self.status_code());
        response_builder.json(json!({
            "error": self.to_string(),
        }))
    }
}
