use crate::app::entity::{prelude::*, user};
use axum::{
    extract::State,
    response::{AppendHeaders, IntoResponse, Redirect},
    routing::get,
    Json, Router,
};

pub fn routes() -> Router<crate::ServeState> {
    Router::new().route(
        "/user",
        get(|State(state): State<crate::ServeState>, claims: crate::serve::jwt::Claims| async move {
            let sub = claims.sub;
            let db_user = User::find()
                .select_only()
                .columns([
                    user::Column::Id,
                    user::Column::Username,
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
}
