use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ClientCore;

use super::JobExecData;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct JobList<T> {
    items: Box<[T]>,
}

impl<T> TryFromResponse for JobList<T>
where
    T: for<'de> Deserialize<'de>,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(JobList {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}")]
pub struct JobListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

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

impl<T> JobListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn active_only(mut self) -> Self {
        self.active_only = true;

        self
    }

    pub fn exec_data(self) -> JobListBuilder<JobList<JobExecData>> {
        JobListBuilder {
            core: self.core,
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
    builder: &JobListBuilder<T>,
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
    builder: &JobListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.exec_data {
        request_builder = request_builder.query(&[("exec-data", "Y")]);
    }

    request_builder
}

fn set_subsystem<T>(mut builder: JobListBuilder<T>, value: Box<str>) -> JobListBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("/-{}", value).into();

    builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs")
            .query(&[
                ("owner", "IBMUSER"),
                ("prefix", "TESTJOB*"),
                ("exec-data", "Y"),
            ])
            .build()
            .unwrap();

        let job_list = zosmf
            .jobs()
            .list()
            .owner("IBMUSER")
            .prefix("TESTJOB*")
            .exec_data()
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_list))
    }
}
