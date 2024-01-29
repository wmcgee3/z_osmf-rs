use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::jobs::{JobData, JobIdentifier};

use super::{JobExecData, JobExecStepData, JobStepData};

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct JobStatusBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(optional, skip_setter, builder_fn = build_exec_data)]
    exec_data: bool,
    #[endpoint(optional, skip_setter, builder_fn = build_step_data)]
    step_data: bool,
    #[endpoint(optional, query = "user-correlator")]
    user_correlator: Option<Box<str>>,

    #[endpoint(optional, skip_builder, skip_setter)]
    target_type: PhantomData<T>,
}

impl JobStatusBuilder<JobData> {
    pub fn exec_data(self) -> JobStatusBuilder<JobExecData> {
        JobStatusBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: true,
            step_data: self.step_data,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }

    pub fn step_data(self) -> JobStatusBuilder<JobStepData> {
        JobStatusBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: true,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl JobStatusBuilder<JobExecData> {
    pub fn step_data(self) -> JobStatusBuilder<JobExecStepData> {
        JobStatusBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: true,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl JobStatusBuilder<JobStepData> {
    pub fn exec_data(self) -> JobStatusBuilder<JobExecStepData> {
        JobStatusBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: true,
            step_data: self.step_data,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

fn build_exec_data<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.step_data {
        request_builder = request_builder.query(&[("exec-data", "Y")]);
    }

    request_builder
}

fn build_step_data<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.step_data {
        request_builder = request_builder.query(&[("step-data", "Y")]);
    }

    request_builder
}

fn set_subsystem<T>(mut builder: JobStatusBuilder<T>, value: Box<str>) -> JobStatusBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}
