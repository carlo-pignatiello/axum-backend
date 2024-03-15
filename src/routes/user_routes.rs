use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse, Json,
};
use chrono::Utc;
use entity::user_account::ActiveModel;
use sea_orm::ActiveValue::{Set, NotSet};
use std::sync::Arc;
use crate::AppState;
use crate::models::user_models::{LoginUserSchema, RegisterUserSchema};
use crate::repos::user_repos::{self, insert_user};
use crate::auth_service::auth_serivce::hash_password;

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let created_at = Utc::now().naive_utc();
    let hashed_password = hash_password(&body.password);
    match hashed_password {
        Ok(h) => {
            let user = ActiveModel {
                username: Set(body.username),
                email: Set(body.email),
                hashed_password: Set(h), 
                password_hashed_algorithm: Set("ES256".to_string()),
                password_salt: Set(data.env.jwt_secret.clone()),
                created_at: Set(created_at),
                id: NotSet,
                role_id: Set(1)
            };
            let response_query = user_repos::insert_user(&data.db, user).await;
            match response_query {
                Ok(i) => {
                    let user_response = serde_json::json!({"user_id": i});
                    Ok((StatusCode::ACCEPTED, Json(user_response)))
                },
                Err(e) => {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Hashing password: {:#?}", e),
                    });
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        },
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Hashing password: {:#?}", e),
            });
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

// pub async fn login_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<LoginUserSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{

// }