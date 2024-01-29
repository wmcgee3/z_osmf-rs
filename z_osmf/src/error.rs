use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing Etag")]
    MissingEtag,

    #[error("missing transaction id")]
    MissingTransactionId,

    #[error("API call failed")]
    ApiError(#[from] reqwest::Error),

    #[error("deserialization of data failed")]
    DeserializationError(#[from] serde::de::value::Error),

    #[error("serialization of data failed")]
    SerializationError(#[from] std::fmt::Error),

    #[error("failed to convert to string")]
    ToStringError(#[from] reqwest::header::ToStrError),

    #[error("an error ocurred {0}")]
    Custom(String),
}
