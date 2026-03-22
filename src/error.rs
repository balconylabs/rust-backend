use axum::{Json, http::status::StatusCode, response::IntoResponse};
use serde_json::json;

pub enum AppError {
    UserAlreadyExists,
    InternalServerError(String),
    ValidationError(validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "A user with that username or email already exists".to_string(),
            ),
            AppError::InternalServerError(err) => {
                println!("Database error: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong on our end.".to_string(),
                )
            }
            AppError::ValidationError(validation_errors) => {
                (StatusCode::BAD_REQUEST, validation_errors.to_string())
            }
        };

        let body = Json(json!({
            "error":error_message
        }));

        (status, body).into_response()
    }
}
