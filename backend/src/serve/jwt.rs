use axum::{async_trait, extract::FromRequestParts, headers, http::request::Parts, RequestPartsExt, TypedHeader};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(jwt_secret.as_bytes())
});

// #[derive(Deserialize)]
// pub struct AuthPayload {
//     pub username: String,
//     pub password: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sub {
    pub user_id: i32,
    pub username: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(flatten)]
    pub sub: Sub,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(cookie) = match parts.extract::<TypedHeader<headers::Cookie>>().await {
            Ok(v) => v,
            Err(e) => return Err(crate::Error::Message(e.to_string())),
        };
        // Decode the user
        if let Some(token) = cookie.get("token") {
            Ok(decode(token)?)
        } else {
            return Err(crate::Error::Message("token not exists".into()));
        }
    }
}

pub fn encode(claims: &Claims) -> anyhow::Result<String> {
    // HS256
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &KEYS.encoding)?;
    Ok(token)
}

pub fn decode(token: &str) -> anyhow::Result<Claims> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let token_message = jsonwebtoken::decode::<Claims>(token, &KEYS.decoding, &validation);
    let claims = token_message?.claims;
    Ok(claims)
}

pub fn build_cookie(key: &str, sub: Sub , max_age: i64) -> anyhow::Result<Cookie<'static>> {
    let jwt_expires_in = max_age;
    let now = chrono::Utc::now();
    let claims = crate::serve::jwt::Claims {
        sub,
        exp: (now + chrono::Duration::days(jwt_expires_in)).timestamp(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };
    let token = crate::serve::jwt::encode(&claims)?;
    Ok(Cookie::build(key.to_owned(), token)
        .path("/")
        .max_age(time::Duration::days(max_age))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish())
}