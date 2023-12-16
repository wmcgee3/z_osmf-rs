pub mod list;
pub mod status;

pub use list::*;
pub use status::*;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct JobsClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl JobsClient {
    pub(crate) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        JobsClient { base_url, client }
    }

    pub fn list(&self) -> JobsListBuilder {
        JobsListBuilder::new(self.base_url.clone(), self.client.clone())
    }
}
