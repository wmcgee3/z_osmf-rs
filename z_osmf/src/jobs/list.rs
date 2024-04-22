use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ClientCore;

use super::{get_subsystem, JobAttributesExec};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(query = "owner")]
    owner: Option<Box<str>>,
    #[endpoint(query = "prefix")]
    prefix: Option<Box<str>>,
    #[endpoint(query = "jobid")]
    job_id: Option<Box<str>>,
    #[endpoint(query = "max-jobs")]
    max_jobs: Option<i32>,
    #[endpoint(query = "user-correlator")]
    user_correlator: Option<Box<str>>,
    #[endpoint(skip_setter, builder_fn = build_exec_data)]
    exec_data: Option<bool>,
    #[endpoint(builder_fn = build_active_only)]
    active_only: Option<bool>,

    target_type: PhantomData<T>,
}

impl<T> JobListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn exec_data(self) -> JobListBuilder<JobList<JobAttributesExec>> {
        JobListBuilder {
            core: self.core,
            subsystem: self.subsystem,
            owner: self.owner,
            prefix: self.prefix,
            job_id: self.job_id,
            max_jobs: self.max_jobs,
            user_correlator: self.user_correlator,
            exec_data: Some(true),
            active_only: self.active_only,
            target_type: PhantomData,
        }
    }
}

fn build_active_only<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.active_only {
        Some(true) => request_builder.query(&[("status", "active")]),
        _ => request_builder,
    }
}

fn build_exec_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.exec_data {
        Some(true) => request_builder.query(&[("exec-data", "Y")]),
        _ => request_builder,
    }
}

fn build_subsystem<T>(builder: &JobListBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
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

    #[test]
    fn subsystem() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/-somesys")
            .build()
            .unwrap();

        let job_list = zosmf
            .jobs()
            .list()
            .subsystem("somesys")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_list))
    }
}
