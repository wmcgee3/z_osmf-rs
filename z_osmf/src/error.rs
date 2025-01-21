use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("z/OSMF API error response: {0:?}")]
    Api(ApiError),
    #[error("data serialization failed: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("invalid response format: {0:?}")]
    InvalidFormat(Arc<[Arc<str>]>),
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
}

#[derive(Debug)]
pub enum ApiError {
    Json {
        url: String,
        status: reqwest::StatusCode,
        category: i32,
        return_code: i32,
        reason: i32,
        message: String,
        details: Option<Vec<String>>,
    },
    Text {
        url: String,
        status: reqwest::StatusCode,
        body: String,
    },
}

impl ApiError {
    pub fn url(&self) -> &str {
        match self {
            Self::Json { url, .. } => url,
            Self::Text { url, .. } => url,
        }
    }

    pub fn status(&self) -> reqwest::StatusCode {
        match self {
            Self::Json { status, .. } => *status,
            Self::Text { status, .. } => *status,
        }
    }
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
                let body = self.text().await.map_err(|_| Error::Reqwest(err))?;
                let ErrorJson {
                    category,
                    return_code,
                    reason,
                    message,
                    details,
                } = serde_json::from_str(&body).map_err(|_| {
                    Error::Api(ApiError::Text {
                        url: url.clone(),
                        status,
                        body,
                    })
                })?;

                return Err(Error::Api(ApiError::Json {
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
