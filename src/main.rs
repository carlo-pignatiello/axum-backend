// use axum::{routing::get, Router};
use sea_orm::{ActiveValue::Set, ActiveValue::NotSet, Database, DatabaseConnection};
mod errors;
mod models;
mod repos;
mod routes;

use repos::user_repos::{insert_user, get_user};
use entity::user_account::ActiveModel;

#[tokio::main]
async fn main() {
    let db_connction = get_database_connection().await;
    let user = get_user(&db_connction, "carloo").await;
    match user {
        Ok(u) => println!("{:?}", u),
        Err(e) => println!("{:?}", e.to_string())
    }
    // build our application with a single route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

async fn get_database_connection() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_connection = Database::connect(db_url).await;
    database_connection.expect("Failed to connect to db")
}
