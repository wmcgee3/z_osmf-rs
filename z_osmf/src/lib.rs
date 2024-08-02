//! # z_osmf
//!
//! The VERY work in progress Rust z/OSMF<sup>TM</sup> Client.
//!
//! ## Examples
//!
//! Create a ZOsmf client:
//! ```
//! # fn example() -> Result<(), z_osmf::Error> {
//! let client = reqwest::Client::new();
//! let zosmf = z_osmf::ZOsmf::new(client, "https://mainframe.my-company.com");
//! # Ok(())
//! # }
//! ```
//!
//! List your datasets:
//! ```
//! # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
//! let my_datasets = zosmf
//!     .datasets()
//!     .list("USERNAME")
//!     .build()
//!     .await?;
//! for dataset in my_datasets.items().iter() {
//!     println!("{}", dataset.name());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! List the files in your home directory:
//! ```
//! # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
//! let my_files = zosmf
//!     .files()
//!     .list("/u/username")
//!     .build()
//!     .await?;
//! for file in my_files.items().iter() {
//!     println!("{}", file.name());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! List all active jobs:
//! ```
//! # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
//! let active_jobs = zosmf
//!     .jobs()
//!     .list()
//!     .owner("*")
//!     .active_only(true)
//!     .build()
//!     .await?;
//! for job in active_jobs.items().iter() {
//!     println!("{}", job.name());
//! }
//! # Ok(())
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

pub use bytes::Bytes;

#[cfg(feature = "datasets")]
pub mod datasets;
#[cfg(feature = "files")]
pub mod files;
#[cfg(feature = "jobs")]
pub mod jobs;
#[cfg(any(feature = "datasets", feature = "files"))]
pub mod restfiles;
#[cfg(feature = "system-variables")]
pub mod system_variables;
#[cfg(feature = "workflows")]
pub mod workflows;

pub use self::error::Error;

pub mod info;

use std::sync::{Arc, RwLock};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use self::error::CheckStatus;
use self::info::{Info, InfoBuilder};

#[cfg(feature = "datasets")]
use self::datasets::DatasetsClient;
#[cfg(feature = "files")]
use self::files::FilesClient;
#[cfg(feature = "jobs")]
use self::jobs::JobsClient;
#[cfg(feature = "system-variables")]
use self::system_variables::SystemVariablesClient;
#[cfg(feature = "workflows")]
use self::workflows::WorkflowsClient;

mod convert;
mod error;
mod utils;

/// # ZOsmf
///
/// Client for interacting with z/OSMF.
///
/// ```
/// # async fn example() -> anyhow::Result<()> {
/// # use z_osmf::ZOsmf;
/// let client = reqwest::Client::new();
/// let url = "https://zosmf.mainframe.my-company.com";
/// let username = "USERNAME";
///
/// let zosmf = ZOsmf::new(client, url);
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
    core: ClientCore,
}

impl ZOsmf {
    /// Create a new z/OSMF client.
    ///
    /// # Example
    /// ```
    /// # async fn example() {
    /// # use z_osmf::ZOsmf;
    /// let client = reqwest::Client::new();
    /// let url = "https://zosmf.mainframe.my-company.com";
    ///
    /// let zosmf = ZOsmf::new(client, url);
    /// # }
    /// ```
    pub fn new<U>(client: reqwest::Client, url: U) -> Self
    where
        U: std::fmt::Display,
    {
        let token = Arc::new(RwLock::new(None));
        let url = url.to_string().into();

        let core = ClientCore { client, token, url };

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
    /// let auth_tokens = zosmf.login("USERNAME", "PASSWORD").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login<U, P>(&self, username: U, password: P) -> Result<Vec<AuthToken>, Error>
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        let response = self
            .core
            .client
            .post(format!("{}/zosmf/services/authenticate", self.core.url))
            .basic_auth(username, Some(password))
            .send()
            .await?
            .check_status()
            .await?;

        let mut tokens: Vec<AuthToken> = response
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .flat_map(|header_value| header_value.try_into().ok())
            .collect();
        tokens.sort_unstable();

        self.set_token(tokens.first().cloned())?;

        Ok(tokens)
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
            .delete(format!("{}/zosmf/services/authenticate", self.core.url))
            .send()
            .await?
            .check_status()
            .await?;

        self.set_token(None)?;

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
        DatasetsClient::new(self.core.clone())
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
        FilesClient::new(self.core.clone())
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
        JobsClient::new(self.core.clone())
    }

    #[cfg(feature = "system-variables")]
    pub fn system_variables(&self) -> SystemVariablesClient {
        SystemVariablesClient::new(self.core.clone())
    }

    #[cfg(feature = "workflows")]
    pub fn workflows(&self) -> WorkflowsClient {
        WorkflowsClient::new(self.core.clone())
    }

    fn set_token(&self, token: Option<AuthToken>) -> Result<(), Error> {
        let mut write = self
            .core
            .token
            .write()
            .map_err(|err| Error::RwLockPoisonError(err.to_string()))?;
        *write = token;

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AuthToken {
    Jwt(Arc<str>),
    Ltpa2(Arc<str>),
}

impl std::str::FromStr for AuthToken {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value) = s.split_once('=').ok_or(Error::InvalidValue(format!(
            "invalid set-cookie header value: {}",
            s
        )))?;

        let token = match name {
            "jwtToken" => AuthToken::Jwt(value.into()),
            "LtpaToken2" => AuthToken::Ltpa2(value.into()),
            _ => return Err(Error::InvalidValue(format!("invalid token name: {}", name))),
        };

        Ok(token)
    }
}

impl TryFrom<&HeaderValue> for AuthToken {
    type Error = crate::Error;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        let s = value.to_str()?;

        s.split_once(';')
            .ok_or(Error::InvalidValue(format!(
                "invalid set-cookie header value: {}",
                s
            )))?
            .0
            .parse()
    }
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AuthToken::Jwt(token) => format!("jwtToken={}", token),
            AuthToken::Ltpa2(token) => format!("LtpaToken2={}", token),
        };

        write!(f, "{}", s)
    }
}

impl From<&AuthToken> for HeaderMap {
    fn from(value: &AuthToken) -> Self {
        let (key, val) = match value {
            AuthToken::Jwt(token_value) => (
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token_value).parse().unwrap(),
            ),
            AuthToken::Ltpa2(token_value) => (
                reqwest::header::COOKIE,
                format!("LtpaToken2={}", token_value).parse().unwrap(),
            ),
        };

        let mut headers = HeaderMap::new();
        headers.insert(key, val);

        headers
    }
}

#[derive(Clone, Debug)]
struct ClientCore {
    client: reqwest::Client,
    token: Arc<RwLock<Option<AuthToken>>>,
    url: Arc<str>,
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
