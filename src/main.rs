use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Build an application with a single route at the root path "/"
    let app = Router::new().route("/", get(handler));

    // Bind strictly to localhost on port 8000 to stay behind the proxy
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    
    println!("Listening on 127.0.0.1:8000...");
    
    // Start the server
    axum::serve(listener, app).await.unwrap();
}

// The function that runs when someone visits your domain
async fn handler() -> &'static str {
    "rust-backend test"
}
