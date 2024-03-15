mod errors;
mod models;
mod repos;
mod routes;
mod auth_service;
mod config;

use config::Config;
use tower_http::cors::CorsLayer;
use axum::{
    Router,
    Json,
    routing::{get, post},
    response::IntoResponse,
    http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}},
};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use routes::{protected_routes, user_routes};

pub struct AppState {
    db: DatabaseConnection,
    env: Config,
}

#[tokio::main]
async fn main() {
    let config = Config::init();
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = router(Arc::new(AppState {
        db: get_database_connection().await,
        env: config.clone(),
    }))
    .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_database_connection() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_connection = Database::connect(db_url).await;
    database_connection.expect("Failed to connect to db")
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_check))
        .route("/api/auth/register", post(user_routes::register_handler))
        // .route("/api/auth/login", post(user_routes::login_handler))
        .route("/api/auth/protected", get(protected_routes::protected_route))
        .with_state(app_state)
}

pub async fn health_check() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "true"
    });
    Json(json_response)
}