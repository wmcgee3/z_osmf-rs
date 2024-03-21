#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use bytes::Bytes;

pub mod error;
pub mod info;

#[cfg(feature = "datasets")]
pub mod datasets;
#[cfg(feature = "files")]
pub mod files;
#[cfg(feature = "jobs")]
pub mod jobs;
#[cfg(feature = "variables")]
pub mod variables;

pub use self::error::Error;

mod convert;
mod utils;

use std::sync::{Arc, RwLock};

use reqwest::header::HeaderValue;

use self::error::CheckStatus;
use self::info::{Info, InfoBuilder};

#[cfg(feature = "datasets")]
use self::datasets::DatasetsClient;
#[cfg(feature = "files")]
use self::files::FilesClient;
#[cfg(feature = "jobs")]
use self::jobs::JobsClient;
#[cfg(feature = "variables")]
use self::variables::VariablesClient;

/// # ZOsmf
///
/// Client for interacting with z/OSMF.
///
/// ```
/// # async fn example() -> anyhow::Result<()> {
/// # use z_osmf::ZOsmf;
/// let client = reqwest::Client::new();
/// let base_url = "https://zosmf.mainframe.my-company.com";
/// let username = "USERNAME";
///
/// let zosmf = ZOsmf::new(client, base_url);
/// zosmf.login(username, "PASSWORD").await?;
///
/// let my_datasets = zosmf.datasets().list(username).build().await?;
///
/// for dataset in my_datasets.items().iter() {
///     println!("{:#?}", dataset);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ZOsmf {
    core: Arc<ClientCore>,
}

impl ZOsmf {
    /// Create a new z/OSMF client.
    ///
    /// # Example
    /// ```
    /// # async fn example() {
    /// # use z_osmf::ZOsmf;
    /// let client = reqwest::Client::new();
    /// let base_url = "https://zosmf.mainframe.my-company.com";
    ///
    /// let zosmf = ZOsmf::new(client, base_url);
    /// # }
    /// ```
    pub fn new<B>(client: reqwest::Client, base_url: B) -> Self
    where
        B: std::fmt::Display,
    {
        let base_url = format!("{}", base_url).trim_end_matches('/').into();
        let core = Arc::new(ClientCore {
            base_url,
            client,
            cookie: RwLock::new(None),
        });

        ZOsmf { core }
    }

    /// Retrieve information about z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let info = zosmf.info().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn info(&self) -> Result<Info, Error> {
        InfoBuilder::new(self.core.clone()).build().await
    }

    /// Authenticate with z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.login("USERNAME", "PASSWORD").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login<U, P>(&self, username: U, password: P) -> Result<(), Error>
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        let response = self
            .core
            .client
            .post(format!(
                "{}/zosmf/services/authenticate",
                self.core.base_url
            ))
            .basic_auth(username, Some(password))
            .send()
            .await?
            .check_status()
            .await?;

        match self.core.cookie.write() {
            Ok(mut cookie) => {
                *cookie = Some(
                    response
                        .headers()
                        .get("Set-Cookie")
                        .ok_or(Error::Custom("failed to get authentication token".into()))?
                        .clone(),
                );
            }
            Err(_) => {
                return Err(Error::Custom(
                    "failed to retrieve authentication header".into(),
                ))
            }
        }

        Ok(())
    }

    /// Logout of z/OSMF.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong> Logging out before an action has completed,
    /// like immediately after submitting a job, can cause the action to fail.
    /// </p>
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.logout().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn logout(&self) -> Result<(), Error> {
        self.core
            .client
            .delete(format!(
                "{}/zosmf/services/authenticate",
                self.core.base_url
            ))
            .send()
            .await?
            .check_status()
            .await?;

        match self.core.cookie.write() {
            Ok(mut cookie) => {
                *cookie = None;
            }
            Err(_) => {
                return Err(Error::Custom(
                    "failed to retrieve authentication header".into(),
                ))
            }
        }

        Ok(())
    }

    /// Create a sub-client for interacting with datasets.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let datasets_client = zosmf.datasets();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "datasets")]
    pub fn datasets(&self) -> DatasetsClient {
        DatasetsClient::new(&self.core)
    }

    /// Create a sub-client for interacting with files.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let files_client = zosmf.files();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "files")]
    pub fn files(&self) -> FilesClient {
        FilesClient::new(&self.core)
    }

    /// Create a sub-client for interacting with jobs.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jobs_client = zosmf.jobs();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "jobs")]
    pub fn jobs(&self) -> JobsClient {
        JobsClient::new(&self.core)
    }

    #[cfg(feature = "variables")]
    pub fn variables(&self) -> VariablesClient {
        VariablesClient::new(self.core.clone())
    }
}

#[derive(Debug)]
struct ClientCore {
    base_url: Box<str>,
    client: reqwest::Client,
    cookie: RwLock<Option<HeaderValue>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn get_zosmf() -> ZOsmf {
        ZOsmf::new(reqwest::Client::new(), "https://test.com")
    }

    pub(crate) trait GetJson {
        fn json(&self) -> Option<serde_json::Value>;
    }

    impl GetJson for reqwest::Request {
        fn json(&self) -> Option<serde_json::Value> {
            Some(
                serde_json::from_slice(self.body()?.as_bytes()?)
                    .expect("failed to deserialize JSON"),
            )
        }
    }
}
