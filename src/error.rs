use axum::{Json, response::IntoResponse};
use serde_json::json;

pub enum AppError {
    UserAlreadyExists,
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::UserAlreadyExists => (
                axum::http::status::StatusCode::CONFLICT,
                "A user with that username or email already exists".to_string(),
            ),
            AppError::InternalServerError(err) => {
                println!("Database error: {err}");
                (
                    axum::http::status::StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong on our end.".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error":error_message
        }));

        (status, body).into_response()
    }
}
