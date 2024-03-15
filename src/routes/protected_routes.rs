use axum::{response::IntoResponse, Json};

pub async fn protected_route() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "type_routes": "protected"
    });
    Json(json_response)
}