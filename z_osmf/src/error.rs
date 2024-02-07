use serde::{Deserialize, Serialize};
use thiserror::Error;
use z_osmf_macros::Getters;

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
    #[error("")]
    Zosmf {
        url: Box<reqwest::Url>,
        status: reqwest::StatusCode,
        json: Box<ErrorJson>,
    },
    #[error("an error ocurred {0}")]
    Custom(String),
}

pub trait CheckStatus {
    fn check_status(self) -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;
}

impl CheckStatus for reqwest::Response {
    async fn check_status(self) -> Result<Self, Error> {
        match self.error_for_status_ref() {
            Ok(_) => {}
            Err(err) => {
                let url = self.url().clone().into();
                let status = self.status();
                let json = self.json().await.map_err(|_| Error::Api(err))?;

                return Err(Error::Zosmf { url, status, json });
            }
        }

        Ok(self)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct ErrorJson {
    #[getter(copy)]
    category: i32,
    #[getter(copy)]
    #[serde(rename = "rc")]
    return_code: i32,
    #[getter(copy)]
    reason: i32,
    message: Box<str>,
    #[serde(default)]
    details: Option<Box<[Box<str>]>>,
}
