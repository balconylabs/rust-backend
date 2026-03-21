use axum::{
    extract::{State, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use std::env;

// 1. Define the structures for our data
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

#[tokio::main]
async fn main() {
    // Load the .env file if it exists (useful for local development)
    dotenvy::dotenv().ok();

    // Grab the connection string
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Set up the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    // 3. Build the router and pass the database pool as shared state
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user))
        .with_state(pool);

    // Bind and serve
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Listening on 127.0.0.1:8000...");
    axum::serve(listener, app).await.unwrap();
}

// The original test route
async fn root_handler() -> &'static str {
    "Hello from the Rust backend! The database pool is live."
}

// 4. The endpoint to create a new user
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, axum::http::StatusCode> {
    
    // sqlx::query_as! checks the SQL syntax and return types against your database at compile time
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
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the newly created user as JSON
    Ok(Json(user))
}