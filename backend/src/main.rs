mod app;
mod error;
mod logger;
use error::Error;

use axum::{routing::get, Router};
use sea_orm::{ConnectOptions, Database, DbConn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::from_filename(".env.local"); // 先加载优先级更高
    dotenvy::from_filename(".env")?;
    logger::logging()?;

    let addr = format!("127.0.0.1:{}", std::env::var("SERVE_PORT")?).parse()?;
    let state = AppState::init().await?;
    let app = Router::new()
        .route("/", get(|| async { "Hello, RustChina!" }))
        .nest("/", app::router::compose())
        .with_state(state);

    log::debug!("GraphiQL IDE: {}/graphql", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {
    github_oauth_url: String,
    db_conn: DbConn,
}

impl AppState {
    async fn init() -> anyhow::Result<Self> {
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
