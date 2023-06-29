use async_graphql::*;

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
