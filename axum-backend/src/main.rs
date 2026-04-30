use axum::{
    routing::{get, post},
    Router,
};
use log;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

mod handlers;
use handlers::{hello, list_names};

/// Initialize the application router with all routes and middleware
fn init_routers() -> Router {
    // Get the path to the dist directory
    // For web frontend builds, this path may need to be adjusted based on build context
    let dist_path = PathBuf::from("web-frontend/dist");
    if !dist_path.exists() {
        log::warn!(
            "Static assets directory NOT FOUND: {:?}",
            dist_path.display()
        );
        log::warn!("Web frontend may not be available. Ensure 'npm run build' has been run in web-frontend/");
    }

    Router::new()
        .route("/api/hello", post(hello))
        .route("/api/list", get(list_names))
        .nest_service(
            "/web",
            ServeDir::new(dist_path.clone()).fallback(ServeFile::new(dist_path.join("index.html"))),
        )
        .layer(TraceLayer::new_for_http())
}

/// Initialize and start the axum server
async fn init_server(app: Router) {
    // Load host and port from environment variables with defaults
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    // Parse host and bind to address
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port))
        .expect("Failed to parse socket address");

    log::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Server encountered an error");
}

/// Initialize logging configuration
fn init_logging() {
    let _ = env_logger::init();
}

/// Load environment variables from .env file
fn init_env() {
    dotenv::dotenv().ok();
}   

#[tokio::main]
async fn main() {
    init_env();
    init_logging();
    let app = init_routers();
    init_server(app).await;
}
