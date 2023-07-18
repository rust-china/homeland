use axum::{
    extract::{Query, State},
    http::header,
    response::{AppendHeaders, IntoResponse, Redirect},
    routing::get,
    Router,
};
use entity::{prelude::*, user};
use std::sync::Arc;

pub fn routes() -> Router<Arc<crate::AppState>> {
    let github_oauth_client_id = std::env::var("GITHUB_OAUTH_CLIENT_ID").unwrap();
    let github_oauth_redirect_url = std::env::var("GITHUB_OAUTH_REDIRECT_URL").unwrap();
    let github_oauth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email",
        github_oauth_client_id, github_oauth_redirect_url
    );

    Router::new()
        .route("/", get(|| async move { Redirect::temporary(&github_oauth_url) }))
        .route("/callback", get(callback))
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}
pub(crate) async fn callback(State(state): State<Arc<crate::AppState>>, query: Query<CallbackQuery>) -> Result<impl IntoResponse, crate::Error> {
    let handle = || async {
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
        log::trace!("https://api.github.com/user => {:?}", resp);

        let username = format!("{}", resp["login"].as_str().ok_or_else(|| crate::Error::RespMessage(422, "username not found".into()))?);
        let email = format!("{}", resp["email"].as_str().ok_or_else(|| crate::Error::RespMessage(422, "email not found".into()))?);
        let name = match resp["name"].as_str() {
            Some(v) => Some(v.to_owned()),
            _ => None
        };
        let mut user = user::ActiveModel {
            name: Set(name),
            username: Set(username.clone()),
            email: Set(email),
            github_data: Set(None), /*Set(Some(resp))*/
            ..Default::default()
        };
        let db_user = User::find()
            .filter(user::Column::Username.eq(username.clone()))
            .into_model::<user::Model>()
            .one(&state.db_conn)
            .await?;
        let ret;
        if let Some(db_user) = db_user {
            user.id = Set(db_user.id);
            ret = user.update(&state.db_conn).await?;
        } else {
            ret = user.insert(&state.db_conn).await?;
        }
        log::trace!("saved: {}", serde_json::json!(ret));

        let max_age: i64 = std::env::var("JWT_MAX_AGE")?.parse::<i64>()?;
        let sub = crate::serve::jwt::Sub {
            user_id: ret.id,
            username: ret.username,
        };
        let token_cookie = crate::serve::jwt::build_cookie("token", sub.clone(), max_age)?;

        return Ok((
            AppendHeaders([(header::SET_COOKIE, token_cookie.to_string())]),
            Redirect::temporary(&github_oauth_success_url),
        ));
    };

    match handle().await {
        Ok(v) => Ok(v),
        Err(_e) => Err(crate::Error::Message("auth error".into())),
    }
}
