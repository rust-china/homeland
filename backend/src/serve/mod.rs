pub mod jwt;

use std::net::SocketAddr;

// use axum::http::{
//     header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
//     HeaderValue, Method,
// };
use axum::{routing::get, Router, Server};
use sea_orm::{ConnectOptions, Database, DbConn};
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    // CorsLayer::new()
    //     .allow_origin(cors::AllowOrigin::any())
    //     .allow_methods(cors::AllowMethods::any())
    //     .allow_headers(cors::AllowHeaders::any())
    //     .allow_credentials(cors::AllowCredentials::yes());

    // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
    // CorsLayer::new()
    //     // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_origin(cors::AllowOrigin::predicate(|_origin: &HeaderValue, _request_head: &axum::http::request::Parts| true))
    //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    //     .allow_credentials(true)
    //     .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    CorsLayer::new()
}

#[derive(Debug, Clone)]
pub struct ServeState {
    pub github_oauth_url: String,
    pub db_conn: DbConn,
}

impl ServeState {
    pub async fn init() -> anyhow::Result<Self> {
        let github_oauth_client_id = std::env::var("GITHUB_OAUTH_CLIENT_ID").unwrap();
        let github_oauth_redirect_url = std::env::var("GITHUB_OAUTH_REDIRECT_URL").unwrap();

        Ok(Self {
            github_oauth_url: format!(
                "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email",
                github_oauth_client_id, github_oauth_redirect_url
            ),
            db_conn: {
                let database_url = std::env::var("DATABASE_URL").unwrap();
                let mut opt = ConnectOptions::new(database_url);
                opt.sqlx_logging(false) // Disabling SQLx log
                    .sqlx_logging_level(log::LevelFilter::Info); // Setting SQLx log level
                Database::connect(opt).await?
            },
        })
    }
}

pub async fn listen(addr: SocketAddr) -> anyhow::Result<()> {
    let state = ServeState::init().await?;
    let app = Router::new()
        .route("/", get(|claims: Option<jwt::Claims>| async move { 
            let mut text = "Welcome".to_string();
            if let Some(claims) = claims {
                text.push_str(&format!(", {:?}", claims.sub));
            }
            text
        }))
        .nest("/", crate::app::router::compose())
        .with_state(state)
        .layer(cors_layer());

    log::debug!("GraphiQL IDE: {}/graphql", &addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
