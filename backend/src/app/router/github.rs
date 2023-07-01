use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use crate::app::entity::{user, prelude::*};

pub fn routes() -> Router<crate::AppState> {
    Router::new()
        .route("/", get(|State(state): State<crate::AppState>| async move { Redirect::temporary(&state.github_oauth_url) }))
        .route("/callback", get(callback))
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}

pub(crate) async fn callback(State(state): State<crate::AppState>, query: Query<CallbackQuery>) -> Result<impl IntoResponse, crate::Error> {
    let github_oauth_client_id = std::env::var("GITHUB_OAUTH_CLIENT_ID").unwrap();
    let github_oauth_client_secret = std::env::var("GITHUB_OAUTH_CLIENT_SECRET").unwrap();
    let github_oauth_success_url = std::env::var("GITHUB_OAUTH_SUCCESS_URL").unwrap();
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
    let mut user = user::ActiveModel {
        name: Set(Some(format!("{}", resp["name"]))),
        username: Set(username.clone()),
        email: Set(format!("{}", resp["email"])),
        github_data: Set(Some(resp)),
        ..Default::default()
    };
    let db_user = User::find()
        // .select_only()
        // .columns([
        //     user::Column::Id,
        // ])
        .filter(user::Column::Username.eq(username.clone()))
        // .into_model::<serde_json::Value>()
        // .into_json()
        .one(&state.db_conn)
        .await?;
    let ret;
    if let Some(db_user) = db_user {
        user.id = Set(db_user.id);
        ret = user.update(&state.db_conn).await?;
    } else {
        ret = user.insert(&state.db_conn).await?;
    }
    log::debug!("{}", serde_json::json!(ret));

    Ok(Redirect::temporary(&github_oauth_success_url))
}
