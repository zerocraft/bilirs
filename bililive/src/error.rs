use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("unknown error")]
    Unknown,
    #[error("Reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("API Deserialize error")]
    APIDeserializeError(#[from] serde_json::Error),
}
