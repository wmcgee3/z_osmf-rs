#![forbid(unsafe_code)]

pub mod if_match;

#[cfg(feature = "datasets")]
pub mod datasets;
#[cfg(feature = "files")]
pub mod files;
#[cfg(feature = "jobs")]
pub mod jobs;

mod utils;

use std::sync::Arc;

use datasets::DatasetsClient;
use files::FilesClient;
use jobs::JobsClient;

#[derive(Clone, Debug)]
pub struct Zosmf {
    /// Attribute that holds the [DatasetsClient] sub-client.
    #[cfg(feature = "datasets")]
    pub datasets: DatasetsClient,
    /// Attribute that holds the [FilesClient] sub-client.
    #[cfg(feature = "files")]
    pub files: FilesClient,
    /// Attribute that holds the [JobsClient] sub-client.
    #[cfg(feature = "jobs")]
    pub jobs: JobsClient,

    base_url: Arc<str>,
    client: reqwest::Client,
}

impl Zosmf {
    pub fn new<B>(client_builder: reqwest::ClientBuilder, base_url: B) -> anyhow::Result<Self>
    where
        B: std::fmt::Display,
    {
        let base_url: Arc<str> = format!("{}", base_url)
            .trim_end_matches('/')
            .to_string()
            .into();
        let client = client_builder.cookie_store(true).build()?;

        Ok(Zosmf {
            #[cfg(feature = "datasets")]
            datasets: DatasetsClient::new(base_url.clone(), client.clone()),
            #[cfg(feature = "files")]
            files: FilesClient::new(base_url.clone(), client.clone()),
            #[cfg(feature = "jobs")]
            jobs: JobsClient::new(base_url.clone(), client.clone()),

            base_url,
            client,
        })
    }

    pub async fn login<U, P>(&self, username: U, password: P) -> anyhow::Result<()>
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        self.client
            .post(format!("{}/zosmf/services/authenticate", self.base_url))
            .basic_auth(username, Some(password))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn logout(&self) -> anyhow::Result<()> {
        self.client
            .delete(format!("{}/zosmf/services/authenticate", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
