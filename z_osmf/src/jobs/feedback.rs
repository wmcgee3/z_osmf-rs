use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ClientCore;

use super::{AsynchronousResponse, JobIdentifier};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
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
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct JobFeedbackBuilder<T, U>
where
    T: TryFromResponse,
    U: Clone + FeedbackJson + Serialize,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(builder_fn = build_data)]
    data: U,
    #[endpoint(optional, skip_setter, skip_builder)]
    asynchronous: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T, U> JobFeedbackBuilder<T, U>
where
    T: TryFromResponse,
    U: Clone + FeedbackJson + Serialize,
{
    pub fn asynchronous(self) -> JobFeedbackBuilder<AsynchronousResponse, U> {
        JobFeedbackBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            data: self.data,
            asynchronous: true,
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ClassJson {
    class: char,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Box<str>>,
}

impl ClassJson {
    pub(super) fn new<C>(class: C) -> Self
    where
        C: Into<char>,
    {
        ClassJson {
            class: class.into(),
            version: None,
        }
    }
}

impl FeedbackJson for ClassJson {
    fn set_version<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Box<str>>,
    {
        self.version = Some(value.into());

        self
    }
}

#[derive(Clone, Serialize)]
pub struct RequestJson {
    request: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Box<str>>,
}

impl RequestJson {
    pub(super) fn new<R>(request: R) -> Self
    where
        R: Into<Box<str>>,
    {
        RequestJson {
            request: request.into(),
            version: None,
        }
    }
}

impl FeedbackJson for RequestJson {
    fn set_version<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Box<str>>,
    {
        self.version = Some(value.into());

        self
    }
}

pub trait FeedbackJson {
    fn set_version<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Box<str>>;
}

fn set_subsystem<T, U>(
    mut builder: JobFeedbackBuilder<T, U>,
    value: Box<str>,
) -> JobFeedbackBuilder<T, U>
where
    T: TryFromResponse,
    U: Clone + FeedbackJson + Serialize,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

fn build_data<T, U>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobFeedbackBuilder<T, U>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
    U: Clone + FeedbackJson + Serialize,
{
    let mut data = builder.data.clone();
    data.set_version(if builder.asynchronous { "1.0" } else { "2.0" });

    request_builder.json(&data)
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

        let identifier = JobIdentifier::NameId("TESTJOB2".into(), "JOB00084".into());

        let job_feedback = zosmf.cancel_job(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }

    #[test]
    fn change_class_example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "class": "A",
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

        let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
        let job_feedback = zosmf
            .change_job_class(identifier, 'A')
            .get_request()
            .unwrap();

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

        let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
        let job_feedback = zosmf.hold_job(identifier).get_request().unwrap();

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

        let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
        let job_feedback = zosmf.release_job(identifier).get_request().unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }
}
