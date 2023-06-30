use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error message: `{0}`")]
    Message(String),
    // #[error("error request parameter: {0}")]
    // ReqParamError(String),
    #[error(transparent)]
    SeaOrmDbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", self.to_string())).into_response()
    }
}
