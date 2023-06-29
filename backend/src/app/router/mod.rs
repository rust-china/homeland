mod graphql;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, extract::{State}};

pub fn compose() -> Router<crate::AppState> {
    Router::new()
    .route("/health", get(health))
    .nest("/graphql", graphql::routes())
}

pub(crate) async fn health(State(_state): State<crate::AppState>) -> impl IntoResponse {
    let json: serde_json::Value = serde_json::from_str(r#"{"healthy":true}"#).unwrap();
    (StatusCode::OK, Json(json))
}