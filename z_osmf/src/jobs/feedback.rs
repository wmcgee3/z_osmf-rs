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
    id: Arc<str>,
    #[serde(rename = "jobname")]
    name: Arc<str>,
    #[serde(rename = "original-jobid")]
    original_id: Option<Arc<str>>,
    owner: Arc<str>,
    member: Arc<str>,
    #[serde(rename = "sysname")]
    system_name: Arc<str>,
    job_correlator: Arc<str>,
    status: Arc<str>,
    internal_code: Option<Arc<str>>,
    message: Option<Arc<str>>,
}

impl TryFromResponse for JobFeedback {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}")]
pub struct JobFeedbackBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Arc<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(builder_fn = build_body)]
    request: &'static str,
    #[endpoint(skip_setter, skip_builder)]
    asynchronous: Option<bool>,

    target_type: PhantomData<T>,
}

impl<T> JobFeedbackBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> JobFeedbackBuilder<()> {
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

        let identifier = JobIdentifier::NameId("TESTJOB2".to_string(), "JOB00084".to_string());

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

        let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
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

        let identifier = JobIdentifier::NameId("TESTJOBW".to_string(), "JOB00023".to_string());
        let job_feedback = zosmf.jobs().release(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }
}
