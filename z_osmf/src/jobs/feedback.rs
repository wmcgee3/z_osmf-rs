use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ClientCore;

use super::{AsynchronousResponse, Identifier};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Feedback {
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

impl TryFromResponse for Feedback {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct FeedbackBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: Identifier,
    #[endpoint(builder_fn = build_body)]
    request: &'static str,
    #[endpoint(optional, skip_setter, skip_builder)]
    asynchronous: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> FeedbackBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> FeedbackBuilder<AsynchronousResponse> {
        FeedbackBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            request: self.request,
            asynchronous: true,
            target_type: PhantomData,
        }
    }
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    version: &'static str,
}

fn set_subsystem<T>(mut builder: FeedbackBuilder<T>, value: Box<str>) -> FeedbackBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FeedbackBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: builder.request,
        version: if builder.asynchronous { "1.0" } else { "2.0" },
    })
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

        let identifier = Identifier::NameId("TESTJOB2".into(), "JOB00084".into());

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

        let identifier = Identifier::NameId("TESTJOBW".into(), "JOB00023".into());
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

        let identifier = Identifier::NameId("TESTJOBW".into(), "JOB00023".into());
        let job_feedback = zosmf.jobs().release(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }
}
