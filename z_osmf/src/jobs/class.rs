use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::jobs::JobIdentifier;
use crate::ClientCore;

use super::get_subsystem;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}")]
pub struct JobChangeClassBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(builder_fn = build_body)]
    class: char,
    #[endpoint(skip_setter, skip_builder)]
    asynchronous: Option<bool>,

    target_type: PhantomData<T>,
}

impl<T> JobChangeClassBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> JobChangeClassBuilder<()> {
        JobChangeClassBuilder {
            core: self.core,
            class: self.class,
            subsystem: self.subsystem,
            identifier: self.identifier,
            asynchronous: Some(true),
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Serialize)]
struct RequestJson {
    class: char,
    version: &'static str,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobChangeClassBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        class: builder.class,
        version: if builder.asynchronous == Some(true) {
            "1.0"
        } else {
            "2.0"
        },
    })
}

fn build_subsystem<T>(builder: &JobChangeClassBuilder<T>) -> String
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
    fn class_example_1() {
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
            .jobs()
            .change_class(identifier, 'A')
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }

    #[test]
    fn subsystem() {
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
            .put("https://test.com/zosmf/restjobs/jobs/-somesys/TESTJOBW/JOB00023")
            .json(&json)
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
        let job_feedback = zosmf
            .jobs()
            .change_class(identifier, 'A')
            .subsystem("somesys")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", job_feedback)
        );

        assert_eq!(manual_request.json(), job_feedback.json())
    }
}
