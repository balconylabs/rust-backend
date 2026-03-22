use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::{CreateUser, User},
};

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id, username, email, created_at
        "#,
        payload.username,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e
            && db_err.code().as_deref() == Some("23505")
        {
            return AppError::UserAlreadyExists;
        }

        AppError::InternalServerError(e.to_string())
    })?;

    Ok(Json(user))
}

pub async fn get_users(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, axum::http::StatusCode> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, created_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
