use crate::app::entity::{prelude::*, user};
use async_graphql::*;

#[derive(Default)]
pub struct AuthMutation;
#[Object]
impl AuthMutation {
    // pub async fn signup(&self, _username: String, _password: String) -> Result<bool> {
    //     // 用户注册
    //     Ok(true)
    // }
    // pub async fn login(&self, _username: String, _password: String) -> Result<String> {
    //     // 用户登录并生成 token
    //     Ok("success".into())
    // }
    pub async fn user(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        if let Some(claims) = ctx.data::<Option<crate::serve::jwt::Claims>>()? {
            let sub = &claims.sub;
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
                .filter(user::Column::Username.eq(sub.username.clone()))
                .into_json()
                .one(&state.db_conn)
                .await?;
            if let Some(db_user) = db_user {
                Ok(db_user)
            } else {
                Err(Error::new_with_source(crate::Error::Message("no user".into())))
            }
        } else {
            Err(Error::new_with_source(crate::Error::Message("not login".into())))
        }
    }
}
