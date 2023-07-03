use crate::app::entity::{prelude::*, user};
use axum::{
    extract::{Query, State},
    http::header,
    response::{AppendHeaders, IntoResponse, Redirect},
    routing::get,
    Router,
};

pub fn routes() -> Router<crate::ServeState> {
    Router::new()
        .route(
            "/",
            get(|State(state): State<crate::ServeState>| async move { Redirect::temporary(&state.github_oauth_url) }),
        )
        .route("/callback", get(callback))
}

#[derive(serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
}
#[derive(sea_orm::FromQueryResult)]
struct DbUser {
    id: i32,
}
pub(crate) async fn callback(State(state): State<crate::ServeState>, query: Query<CallbackQuery>) -> Result<impl IntoResponse, crate::Error> {
    let handle  = || async {
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
    
        let username = format!("{}", resp["login"].as_str().unwrap_or(""));
        let mut user = user::ActiveModel {
            name: Set(Some(format!("{}", resp["name"].as_str().unwrap_or("")))),
            username: Set(username.clone()),
            email: Set(format!("{}", resp["email"].as_str().unwrap_or(""))),
            github_data: Set(Some(resp)),
            ..Default::default()
        };
        let db_user = User::find()
            .select_only()
            .columns([user::Column::Id])
            .filter(user::Column::Username.eq(username.clone()))
            .into_model::<DbUser>()
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
        log::trace!("saved: {}", serde_json::json!(ret));
    
        let max_age: i64 = std::env::var("JWT_MAX_AGE")?.parse::<i64>()?;
        let sub = crate::serve::jwt::Sub { user_id: ret.id, username: ret.username };
        let token_cookie = crate::serve::jwt::build_cookie("token", sub.clone(), max_age)?;
    
        return Ok((AppendHeaders([(header::SET_COOKIE, token_cookie.to_string())]), Redirect::temporary(&github_oauth_success_url)))
    };

    match handle().await {
        Ok(v) => Ok(v),
        Err(_e) => Err(crate::Error::Message("auth error".into()))
    }

    
}
