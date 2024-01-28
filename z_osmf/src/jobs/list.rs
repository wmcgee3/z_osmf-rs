use std::sync::Arc;

use serde::Deserialize;
use tokio::runtime::Handle;
use z_osmf_macros::{Endpoint, Getters};

use crate::error::Error;
use crate::jobs::JobData;

#[derive(Clone, Debug, Getters)]
pub struct JobsList<T> {
    items: Box<[T]>,
}

impl<T> TryFrom<reqwest::Response> for JobsList<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let items = Handle::current().block_on(value.json())?;

        Ok(JobsList { items })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs")]
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

    pub async fn build(self) -> Result<JobsList<JobData>, Error> {
        let response = self.get_response().await?;

        response.try_into()
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
