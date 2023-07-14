pub mod schema;
pub mod router;
pub use entity;

use sea_orm::{ConnectOptions, Database, DbConn};
use tokio::sync::OnceCell;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DbConn,
    pub schema: schema::AppSchema,
		pub oss_client: backend::oss_client::OssClient,
}

impl AppState {
	pub async fn init() -> anyhow::Result<Self> {
			Ok(Self {
					db_conn: {
							let database_url = std::env::var("DATABASE_URL")?;
							let mut opt = ConnectOptions::new(database_url);
							opt.sqlx_logging(false) // Disabling SQLx log
									.sqlx_logging_level(log::LevelFilter::Info); // Setting SQLx log level
							Database::connect(opt).await?
					},
					schema: schema::build_schema(),
					oss_client: backend::oss_client::OssClient::from_env()?,
			})
	}
}

pub static APP_STATE: OnceCell<AppState> = OnceCell::const_new();
