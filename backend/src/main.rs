mod app;
mod serve;
mod error;
mod logger;

use app::{AppState, APP_STATE};
use error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename_override(".env")?;
    let _ = dotenvy::from_filename_override(".env.local");
    logger::logging()?;
    
    let _ = app::APP_STATE.get_or_try_init(|| async move {
        AppState::init().await
    }).await;

    let addr = format!("127.0.0.1:{}", std::env::var("SERVE_PORT")?).parse()?;
    serve::listen(addr).await
}
