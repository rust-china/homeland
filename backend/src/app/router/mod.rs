mod github;
mod graphql;
mod auth;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

pub fn compose() -> Router<crate::ServeState> {
    Router::new()
        .route("/health", get(health))
        .nest("/github", github::routes())
        .nest("/graphql", graphql::routes())
        .nest("/auth", auth::routes())
}

pub(crate) async fn health(State(_state): State<crate::ServeState>) -> impl IntoResponse {
    let json: serde_json::Value = serde_json::from_str(r#"{"healthy":true}"#).unwrap();
    (StatusCode::OK, Json(json))
}
