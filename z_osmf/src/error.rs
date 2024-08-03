use serde::Deserialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("data serialization failed: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("invalid response format: {0:?}")]
    InvalidFormat(Box<[Box<str>]>),
    #[error("invalid value: {0}")]
    InvalidValue(String),
    #[error("missing etag")]
    NoEtag,
    #[error("missing transaction id")]
    NoTransactionId,
    #[error("failed to parse int: {0}")]
    NumParseInt(#[from] std::num::ParseIntError),
    #[error("invalid record range: {0}")]
    RecordRange(String),
    #[error("API call failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("poisoned read-write lock: {0}")]
    RwLockPoisonError(String),
    #[error("data deserialization failed: {0}")]
    SerdeDe(#[from] serde::de::value::Error),
    #[error("header value to string failed: {0}")]
    ReqwestHeaderToString(#[from] reqwest::header::ToStrError),
    #[error("z/OSMF error response: {0:?}")]
    ZOsmf(ZOsmfError),
}

#[derive(Debug)]
pub enum ZOsmfError {
    Json {
        url: String,
        status: reqwest::StatusCode,
        category: i32,
        return_code: i32,
        reason: i32,
        message: String,
        details: Option<Vec<String>>,
    },
    Text(String),
}

pub trait CheckStatus {
    fn check_status(self) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        Self: Sized;
}

impl CheckStatus for reqwest::Response {
    async fn check_status(self) -> Result<Self> {
        match self.error_for_status_ref() {
            Ok(_) => {}
            Err(err) => {
                let url = self.url().to_string();
                let status = self.status();
                let text = self.text().await.map_err(|_| Error::Reqwest(err))?;
                let ErrorJson {
                    category,
                    return_code,
                    reason,
                    message,
                    details,
                } = serde_json::from_str(&text)
                    .map_err(|_| Error::ZOsmf(ZOsmfError::Text(text)))?;

                return Err(Error::ZOsmf(ZOsmfError::Json {
                    url,
                    status,
                    category,
                    return_code,
                    reason,
                    message,
                    details,
                }));
            }
        }

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
struct ErrorJson {
    category: i32,
    #[serde(rename = "rc")]
    return_code: i32,
    reason: i32,
    message: String,
    #[serde(default)]
    details: Option<Vec<String>>,
}
