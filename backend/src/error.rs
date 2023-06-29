#[allow(dead_code)]

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error request parameter: {0}")]
    ReqParamError(String),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("error message: `{0}`")]
    Message(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
