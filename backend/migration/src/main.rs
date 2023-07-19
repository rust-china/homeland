use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename_override(".env")?;
    let _ = dotenvy::from_filename_override(".env.local");

    cli::run_cli(migration::Migrator).await;

    Ok(())
}
