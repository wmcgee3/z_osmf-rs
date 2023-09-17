#![forbid(unsafe_code)]

pub mod data_type;
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
    base_url: Arc<str>,
    client: reqwest::Client,

    #[cfg(feature = "datasets")]
    pub datasets: DatasetsClient,
    #[cfg(feature = "files")]
    pub files: FilesClient,
    #[cfg(feature = "jobs")]
    pub jobs: JobsClient,
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

        #[cfg(feature = "datasets")]
        let datasets = DatasetsClient::new(base_url.clone(), client.clone());
        #[cfg(feature = "files")]
        let files = FilesClient::new(base_url.clone(), client.clone());
        #[cfg(feature = "jobs")]
        let jobs = JobsClient::new(base_url.clone(), client.clone());

        Ok(Zosmf {
            base_url,
            client,
            #[cfg(feature = "datasets")]
            datasets,
            #[cfg(feature = "files")]
            files,
            #[cfg(feature = "jobs")]
            jobs,
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
