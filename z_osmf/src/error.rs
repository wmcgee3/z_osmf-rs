use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing Etag")]
    Etag,
    #[error("missing transaction id")]
    TransactionId,
    #[error("API call failed: {0}")]
    Api(#[from] reqwest::Error),
    #[error("deserialization of data failed")]
    Deserialization(#[from] serde::de::value::Error),
    #[error("serialization of data failed")]
    Serialization(#[from] std::fmt::Error),
    #[error("failed to convert to string")]
    ToString(#[from] reqwest::header::ToStrError),
    #[error("an error ocurred {0}")]
    Custom(String),
}
