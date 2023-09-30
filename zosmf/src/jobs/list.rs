use std::sync::Arc;

use zosmf_core::jobs::JobData;
use zosmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct JobsList<T> {
    items: Box<[T]>,
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs")]
pub struct JobsListBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, query = "owner")]
    owner: Option<Box<str>>,
    #[endpoint(optional, query = "prefix")]
    prefix: Option<Box<str>>,
    #[endpoint(optional, query = "jobid")]
    job_id: Option<Box<str>>,
    #[endpoint(optional, query = "max-jobs")]
    max_jobs: Option<i32>,
    #[endpoint(optional, query = "user-correlator")]
    user_correlator: Option<Box<str>>,
    #[endpoint(optional, builder_fn = "build_exec_data")]
    exec_data: Option<bool>,
    #[endpoint(optional, skip_setter, builder_fn = "build_active_only")]
    active_only: bool,
}

impl JobsListBuilder {
    pub fn active_only(mut self) -> Self {
        self.active_only = true;

        self
    }

    pub async fn build(self) -> anyhow::Result<JobsList<JobData>> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        let items = response.json::<Vec<JobData>>().await?.into();

        Ok(JobsList {
            items,
            transaction_id,
        })
    }
}

fn build_active_only(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobsListBuilder,
) -> reqwest::RequestBuilder {
    if builder.active_only {
        request_builder = request_builder.query(&[("status", "active")]);
    }

    request_builder
}

fn build_exec_data(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobsListBuilder,
) -> reqwest::RequestBuilder {
    if let Some(exec_data) = builder.exec_data {
        request_builder =
            request_builder.query(&[("exec-data", if exec_data { "Y" } else { "N" })]);
    }

    request_builder
}
