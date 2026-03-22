use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 30))]
    #[validate(custom(function = "validate_username"))]
    pub username: String,

    #[validate(email)]
    pub email: String,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if !username.chars().next().unwrap().is_ascii_alphabetic()
        || username.chars().any(|c| !c.is_ascii_alphanumeric())
    {
        return Err(ValidationError::new(
            "Username must be alphanumberic and begin with a letter",
        ));
    }

    Ok(())
}
