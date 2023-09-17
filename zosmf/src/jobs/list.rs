use std::sync::Arc;

use zosmf_macros::{Endpoint, Getters};

#[derive(Clone, Debug, Getters)]
pub struct JobsList {}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs")]
pub struct JobsListBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl JobsListBuilder {
    pub async fn build(self) -> anyhow::Result<JobsList> {
        let _response = self.get_response().await?;

        Ok(JobsList {})
    }
}
