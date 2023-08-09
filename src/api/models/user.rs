use secrecy::Secret;
use validator::Validate;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: Secret<String>,
    pub name: String,
    pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub description: String,
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    pub bearer: String,
    pub user: AuthUser,
}

impl From<User> for AuthUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            name: user.name,
            description: user.description,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3), length(max = 20))]
    pub username: String,
    pub first_name: String,
    pub description: String,
    pub last_name: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), validator::ValidationError> {
    if password.len() < 8 {
        return Err(validator::ValidationError::new("Password too short."));
    }
    // Password contains at least one digit
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(validator::ValidationError::new(
            "Password must contain at least one digit.",
        ));
    }
    // Password contains at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(validator::ValidationError::new(
            "Password must contain at least one lowercase letter.",
        ));
    }
    // Password contains at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(validator::ValidationError::new(
            "Password must contain at least one uppercase letter.",
        ));
    }
    // Password contains at least one special character
    if password.chars().all(|c| c.is_alphanumeric()) {
        return Err(validator::ValidationError::new(
            "Password must contain at least one special character.",
        ));
    }
    Ok(())
}
