#![forbid(unsafe_code)]

pub mod data_type;
pub mod datasets;
pub mod files;

mod utils;

use reqwest::{Client, ClientBuilder};

use datasets::DatasetsClient;
use files::FilesClient;

#[derive(Clone, Debug)]
pub struct Zosmf {
    base_url: String,
    client: Client,
}

impl Zosmf {
    pub fn new<B>(client_builder: ClientBuilder, base_url: B) -> anyhow::Result<Self>
    where
        B: std::fmt::Display,
    {
        let base_url = format!("{}", base_url).trim_end_matches('/').to_string();
        let client = client_builder.cookie_store(true).build()?;

        Ok(Zosmf { base_url, client })
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

    pub fn datasets(&self) -> DatasetsClient {
        DatasetsClient::new(&self.base_url, &self.client)
    }

    pub fn files(&self) -> FilesClient {
        FilesClient::new(&self.base_url, &self.client)
    }
}
