use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{JobData, JobDataExec, JobDataExecStep, JobDataStep, JobIdentifier};

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct StatusBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

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

impl StatusBuilder<JobData> {
    pub fn exec_data(self) -> StatusBuilder<JobDataExec> {
        StatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: true,
            step_data: self.step_data,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }

    pub fn step_data(self) -> StatusBuilder<JobDataStep> {
        StatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: true,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl StatusBuilder<JobDataExec> {
    pub fn step_data(self) -> StatusBuilder<JobDataExecStep> {
        StatusBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            exec_data: self.exec_data,
            step_data: true,
            user_correlator: self.user_correlator,
            target_type: PhantomData,
        }
    }
}

impl StatusBuilder<JobDataStep> {
    pub fn exec_data(self) -> StatusBuilder<JobDataExecStep> {
        StatusBuilder {
            core: self.core,
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
    request_builder: reqwest::RequestBuilder,
    builder: &StatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.exec_data {
        true => request_builder.query(&[("exec-data", "Y")]),
        false => request_builder,
    }
}

fn build_step_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &StatusBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.step_data {
        true => request_builder.query(&[("step-data", "Y")]),
        false => request_builder,
    }
}

fn set_subsystem<T>(mut builder: StatusBuilder<T>, value: Box<str>) -> StatusBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
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

        let identifier = JobIdentifier::NameId("BLSJPRMI".into(), "STC00052".into());
        let job_status = zosmf
            .jobs()
            .status(identifier)
            .exec_data()
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_status))
    }
}
