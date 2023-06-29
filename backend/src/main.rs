mod logger;
mod error;
mod app;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::logging()?;
    dotenvy::dotenv()?;

    let addr = format!("127.0.0.1:{}", std::env::var("PORT")?).parse()?;
    let state = AppState::default();
    let app = Router::new()
        .route("/", get(|| async { "Hello, RustChina!" }))
        .nest("/", app::router::compose())
        .with_state(state);

    log::debug!("GraphiQL IDE: {}/graphql", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}


#[derive(Debug, Clone, Default)]
pub struct AppState {}