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
pub struct JobStatusBuilder<'a, T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier<'a>,
    #[endpoint(skip_setter, builder_fn = build_exec_data)]
    exec_data: Option<bool>,
    #[endpoint(skip_setter, builder_fn = build_step_data)]
    step_data: Option<bool>,
    #[endpoint(query = "user-correlator")]
    user_correlator: Option<Box<str>>,

    target_type: PhantomData<T>,
}

impl<'a> JobStatusBuilder<'a, JobAttributes> {
    pub fn exec_data(self) -> JobStatusBuilder<'a, JobAttributesExec> {
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

    pub fn step_data(self) -> JobStatusBuilder<'a, JobAttributesStep> {
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

impl<'a> JobStatusBuilder<'a, JobAttributesExec> {
    pub fn step_data(self) -> JobStatusBuilder<'a, JobAttributesExecStep> {
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

impl<'a> JobStatusBuilder<'a, JobAttributesStep> {
    pub fn exec_data(self) -> JobStatusBuilder<'a, JobAttributesExecStep> {
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

        let identifier = JobIdentifier::NameId("BLSJPRMI", "STC00052");
        let job_status = zosmf
            .jobs()
            .status(identifier)
            .exec_data()
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_status))
    }
}
