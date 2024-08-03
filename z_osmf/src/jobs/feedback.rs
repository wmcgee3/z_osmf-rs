use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

use super::{get_subsystem, JobIdentifier};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFeedback {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    #[serde(rename = "original-jobid")]
    original_id: Option<Box<str>>,
    owner: Box<str>,
    member: Box<str>,
    #[serde(rename = "sysname")]
    system_name: Box<str>,
    job_correlator: Box<str>,
    status: Box<str>,
    internal_code: Option<Box<str>>,
    message: Option<Box<str>>,
}

impl TryFromResponse for JobFeedback {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}")]
pub struct JobFeedbackBuilder<'a, T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier<'a>,
    #[endpoint(builder_fn = build_body)]
    request: &'static str,
    #[endpoint(skip_setter, skip_builder)]
    asynchronous: Option<bool>,

    target_type: PhantomData<T>,
}

impl<'a, T> JobFeedbackBuilder<'a, T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> JobFeedbackBuilder<'a, ()> {
        JobFeedbackBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            request: self.request,
            asynchronous: Some(true),
            target_type: PhantomData,
        }
    }
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    version: &'static str,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobFeedbackBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: builder.request,
        version: if builder.asynchronous == Some(true) {
            "1.0"
        } else {
            "2.0"
        },
    })
}

fn build_subsystem<T>(builder: &JobFeedbackBuilder<T>) -> String
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
    fn cancel_example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "request": "cancel",
            "version": "2.0"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restjobs/jobs/TESTJOB2/JOB00084")
            .json(&json)
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOB2", "JOB00084");

        let job_feedback = zosmf.jobs().cancel(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }

    #[test]
    fn hold_example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "request": "hold",
            "version": "2.0"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restjobs/jobs/TESTJOBW/JOB00023")
            .json(&json)
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00023");
        let job_feedback = zosmf.jobs().hold(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }

    #[test]
    fn release_example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "request": "release",
            "version": "2.0"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restjobs/jobs/TESTJOBW/JOB00023")
            .json(&json)
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBW", "JOB00023");
        let job_feedback = zosmf.jobs().release(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }
}
