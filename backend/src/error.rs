use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    // #[error("error request parameter: {0}")]
    // ReqParamError(String),
    #[error(transparent)]
    SeaOrmDbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ParseNumError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, serde::Serialize)]
struct ErrorJson {
    code: i32,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let json = ErrorJson {
            code: 422,
            message: format!("Error: {}", self.to_string()),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
    }
}
