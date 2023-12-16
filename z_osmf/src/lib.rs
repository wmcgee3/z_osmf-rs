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
pub struct ZOsmf {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl ZOsmf {
    pub fn new<B>(client_builder: reqwest::ClientBuilder, base_url: B) -> anyhow::Result<Self>
    where
        B: std::fmt::Display,
    {
        let base_url: Arc<str> = format!("{}", base_url)
            .trim_end_matches('/')
            .to_string()
            .into();
        let client = client_builder.cookie_store(true).build()?;

        Ok(ZOsmf { base_url, client })
    }

    pub async fn login<U, P>(&self, username: U, password: P) -> anyhow::Result<()>
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        self.client
            .post(format!("{}/z_osmf/services/authenticate", self.base_url))
            .basic_auth(username, Some(password))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn logout(&self) -> anyhow::Result<()> {
        self.client
            .delete(format!("{}/z_osmf/services/authenticate", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    #[cfg(feature = "datasets")]
    pub fn datasets(self) -> DatasetsClient {
        DatasetsClient::new(self.base_url.clone(), self.client.clone())
    }

    #[cfg(feature = "files")]
    pub fn files(self) -> FilesClient {
        FilesClient::new(self.base_url.clone(), self.client.clone())
    }

    #[cfg(feature = "jobs")]
    pub fn jobs(self) -> JobsClient {
        JobsClient::new(self.base_url.clone(), self.client.clone())
    }
}
