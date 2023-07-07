use async_graphql::*;
use entity::{prelude::*, user};

#[derive(Default, Debug)]
pub struct AuthQuery;
#[Object]
impl AuthQuery {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<serde_json::Value> {
        let state = ctx.data::<crate::AppState>()?;
        let claims = ctx
            .data::<Option<crate::serve::jwt::Claims>>()?
            .as_ref()
            .ok_or_else(|| crate::Error::Message("should login".into()))
            .map_err(|e| e.extend_with(|_, e| e.set("code", 401)))?;

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
            .await?
            .ok_or_else(|| crate::Error::Message("user not exists".into()))?;
        Ok(db_user)
    }
}

#[derive(Default)]
pub struct AuthMutation;
#[Object]
impl AuthMutation {
    pub async fn signup(&self, _username: String, _password: String) -> Result<bool> {
        // 用户注册
        Ok(true)
    }
    pub async fn login(&self, _username: String, _password: String) -> Result<String> {
        // 用户登录并生成 token
        Ok("success".into())
    }
}
