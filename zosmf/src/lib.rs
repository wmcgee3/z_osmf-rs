#![forbid(unsafe_code)]

pub mod datasets;

use anyhow::Result;
use datasets::Datasets;
use reqwest::{Client, ClientBuilder};

#[derive(Clone, Debug)]
pub struct Zosmf {
    base_url: String,
    client: Client,
}

impl Zosmf {
    pub async fn new<B>(client_builder: ClientBuilder, base_url: B) -> Result<Self>
    where
        B: std::fmt::Display,
    {
        let base_url = format!("{}", base_url).trim_end_matches('/').to_string();
        let client = client_builder.cookie_store(true).build()?;

        Ok(Zosmf { base_url, client })
    }

    pub async fn login<U, P>(&self, username: U, password: P) -> Result<()>
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

    pub async fn logout(&self) -> Result<()> {
        self.client
            .delete(format!("{}/zosmf/services/authenticate", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub fn datasets(&self) -> Datasets {
        Datasets::new(&self.base_url, &self.client)
    }
}
