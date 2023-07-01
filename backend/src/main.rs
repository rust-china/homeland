mod app;
mod serve;
mod error;
mod logger;
use serve::ServeState;
use error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::from_filename(".env.local"); // 先加载优先级更高
    dotenvy::from_filename(".env")?;
    logger::logging()?;

    let addr = format!("127.0.0.1:{}", std::env::var("SERVE_PORT")?).parse()?;
    serve::listen(addr).await
}
