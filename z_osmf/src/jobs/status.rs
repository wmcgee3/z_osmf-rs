use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{
    get_subsystem, JobAttributes, JobAttributesExec, JobAttributesExecStep, JobAttributesStep,
    JobIdentifier,
};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}")]
pub struct JobStatusBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Arc<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(skip_setter, builder_fn = build_exec_data)]
    exec_data: Option<bool>,
    #[endpoint(skip_setter, builder_fn = build_step_data)]
    step_data: Option<bool>,
    #[endpoint(query = "user-correlator")]
    user_correlator: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

impl JobStatusBuilder<JobAttributes> {
    pub fn exec_data(self) -> JobStatusBuilder<JobAttributesExec> {
        JobStatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: Some(true),
            step_data: self.step_data,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }

    pub fn step_data(self) -> JobStatusBuilder<JobAttributesStep> {
        JobStatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: Some(true),
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl JobStatusBuilder<JobAttributesExec> {
    pub fn step_data(self) -> JobStatusBuilder<JobAttributesExecStep> {
        JobStatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: Some(true),
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl JobStatusBuilder<JobAttributesStep> {
    pub fn exec_data(self) -> JobStatusBuilder<JobAttributesExecStep> {
        JobStatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: Some(true),
            step_data: self.step_data,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

fn build_exec_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.exec_data {
        Some(true) => request_builder.query(&[("exec-data", "Y")]),
        _ => request_builder,
    }
}

fn build_step_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobStatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.step_data {
        Some(true) => request_builder.query(&[("step-data", "Y")]),
        _ => request_builder,
    }
}

fn build_subsystem<T>(builder: &JobStatusBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    use super::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/BLSJPRMI/STC00052")
            .query(&[("exec-data", "Y")])
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("BLSJPRMI".to_string(), "STC00052".to_string());
        let job_status = zosmf
            .jobs()
            .status(identifier)
            .exec_data()
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_status))
    }
}
