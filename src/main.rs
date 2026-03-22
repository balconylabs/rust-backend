mod error;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::env;

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
        .route(
            "/users",
            post(routes::users::create_user).get(routes::users::get_users),
        )
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
