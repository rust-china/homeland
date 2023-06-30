use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::from_filename(".env.local"); // 先加载优先级更高
    dotenvy::from_filename(".env")?;

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
