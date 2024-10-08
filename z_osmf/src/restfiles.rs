//! Functionality shared between the datasets and files modules

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Getters;

use crate::convert::TryFromResponse;
use crate::{Error, Result};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CopyDataType {
    Binary,
    Executable,
    Text,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Etag {
    etag: Option<Arc<str>>,
    transaction_id: Arc<str>,
}

impl TryFromResponse for Etag {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let etag = get_etag(&value)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(Etag {
            etag,
            transaction_id,
        })
    }
}

pub(crate) fn get_etag(response: &reqwest::Response) -> Result<Option<Arc<str>>> {
    Ok(response
        .headers()
        .get("Etag")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

pub(crate) fn get_transaction_id(response: &reqwest::Response) -> Result<Arc<str>> {
    Ok(response
        .headers()
        .get("X-IBM-Txid")
        .ok_or(Error::NoTransactionId)?
        .to_str()?
        .into())
}

impl TryFromResponse for String {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        get_transaction_id(&value).map(|v| v.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_etag() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("Etag", "1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_etag(&response).unwrap(), Some("1234".into()));

        let response = reqwest::Response::from(http::Response::new(""));
        assert_eq!(get_etag(&response).unwrap(), None);
    }

    #[test]
    fn test_get_transaction_id() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("X-IBM-Txid", "1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_transaction_id(&response).unwrap(), "1234".into());

        let response = reqwest::Response::from(http::Response::new(""));
        assert!(get_transaction_id(&response).is_err());
    }
}
