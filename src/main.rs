use axum::{
    Router,
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::env;

#[derive(Serialize, FromRow)]
struct User {
    id: uuid::Uuid,
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

enum AppError {
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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user).get(get_users))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on 127.0.0.1:8000...");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello from the Rust backend! The database pool is live."
}

async fn create_user(
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

async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, axum::http::StatusCode> {
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
