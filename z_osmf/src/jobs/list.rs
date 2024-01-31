use std::marker::PhantomData;
use std::sync::Arc;

use serde::Deserialize;
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::jobs::JobExecData;

#[derive(Clone, Debug)]
pub struct JobsList<T> {
    pub items: Box<[T]>,
}

impl<T> TryFromResponse for JobsList<T>
where
    T: for<'de> Deserialize<'de>,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(JobsList {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}")]
pub struct JobsListBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
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
    #[endpoint(optional, skip_setter, builder_fn = build_exec_data)]
    exec_data: bool,
    #[endpoint(optional, skip_setter, builder_fn = build_active_only)]
    active_only: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> JobsListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn active_only(mut self) -> Self {
        self.active_only = true;

        self
    }

    pub fn exec_data(self) -> JobsListBuilder<JobsList<JobExecData>> {
        JobsListBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            owner: self.owner,
            prefix: self.prefix,
            job_id: self.job_id,
            max_jobs: self.max_jobs,
            user_correlator: self.user_correlator,
            exec_data: true,
            active_only: self.active_only,
            target_type: PhantomData,
        }
    }
}

fn build_active_only<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobsListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.active_only {
        request_builder = request_builder.query(&[("status", "active")]);
    }

    request_builder
}

fn build_exec_data<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobsListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.exec_data {
        request_builder = request_builder.query(&[("exec-data", "Y")]);
    }

    request_builder
}

fn set_subsystem<T>(mut builder: JobsListBuilder<T>, value: Box<str>) -> JobsListBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("/-{}", value).into();

    builder
}
