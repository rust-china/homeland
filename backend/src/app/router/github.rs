use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
    routing::get,
    Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set};

use crate::app::entity;

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route(
            "/",
            get(|State(state): State<crate::AppState>| async move { Redirect::temporary(&state.github_oauth_url) }),
        )
        .route("/callback", get(callback))
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}

pub(crate) async fn callback(State(state): State<crate::AppState>, query: Query<CallbackQuery>) -> Result<impl IntoResponse, crate::Error> {
    let github_oauth_client_id = std::env::var("GITHUB_OAUTH_CLIENT_ID").unwrap();
    let github_oauth_client_secret = std::env::var("GITHUB_OAUTH_CLIENT_SECRET").unwrap();
    let client = reqwest::Client::new();
    let resp = client
        .get("https://github.com/login/oauth/access_token")
        .query(&[
            ("client_id", github_oauth_client_id),
            ("client_secret", github_oauth_client_secret),
            ("code", query.code.to_owned()),
        ])
        .send()
        .await?
        .text()
        .await?;
    log::trace!("https://github.com/login/oauth/access_token => {:?}", resp);
    let params: std::collections::HashMap<String, String> = serde_qs::from_str(&resp)?;

    let access_token = match params.get("access_token") {
        Some(value) => value,
        None => return Err(crate::Error::Message(resp)),
    };

    let resp = client
        .get("https://api.github.com/user")
        .header("User-Agent", "Awesome-Octocat-App")
        .header("Authorization", format!("token {}", access_token))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    log::trace!("hhttps://api.github.com/user => {:?}", resp);

    let username = format!("{}", resp["login"]);
    let user = entity::user::Entity::find()
        .select_only()
        .columns([
            entity::user::Column::Id,
            entity::user::Column::Name,
            entity::user::Column::Username,
            entity::user::Column::Email,
        ])
        .filter(entity::user::Column::Username.eq(username.clone()))
        .into_model::<serde_json::Value>()
        .one(&state.db_conn)
        .await?;
    if let Some(user) = user {
        Ok((StatusCode::OK, Json(user)))
    } else {
        let user = entity::user::ActiveModel {
            name: Set(Some(format!("{}", resp["name"]))),
            username: Set(username.clone()),
            email: Set(format!("{}", resp["email"])),
            github_extra: Set(Some(resp)),
            ..Default::default()
        };
        let user: entity::user::Model = user.insert(&state.db_conn).await?;
        Ok((StatusCode::OK, Json(serde_json::json!(user))))
    }
}
