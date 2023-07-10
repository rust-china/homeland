use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use entity::{prelude::*, user};

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/user",
            get(|State(state): State<crate::AppState>, claims: crate::serve::jwt::Claims| async move {
                let sub = claims.sub;
                let db_user = User::find()
                    .select_only()
                    .columns([
                        user::Column::Id,
                        user::Column::Username,
                        user::Column::Name,
                        user::Column::Email,
                        user::Column::CreatedAt,
                        user::Column::UpdatedAt,
                    ])
                    .filter(user::Column::Username.eq(sub.username))
                    .into_json()
                    .one(&state.db_conn)
                    .await?;
                if let Some(db_user) = db_user {
                    Ok(Json(db_user))
                } else {
                    Err(crate::Error::Message("no user".into()))
                }
            }),
        )
        .route(
            "/logout",
            post(|jar: CookieJar| async move {
                let _ = jar.remove(Cookie::named("cookie"));
                let json: serde_json::Value = serde_json::from_str(r#"{"code":200}"#).unwrap();
                (StatusCode::OK, Json(json))
            }),
        )
}
