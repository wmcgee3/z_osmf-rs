use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::jobs::Identifier;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}")]
pub struct ClassBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: Identifier,
    #[endpoint(builder_fn = build_body)]
    class: char,
    #[endpoint(optional, skip_setter, skip_builder)]
    asynchronous: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> ClassBuilder<T>
where
    T: TryFromResponse,
{
    pub fn asynchronous(self) -> ClassBuilder<()> {
        ClassBuilder {
            core: self.core,
            class: self.class,
            subsystem: self.subsystem,
            identifier: self.identifier,
            asynchronous: true,
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
    builder: &ClassBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        class: builder.class,
        version: if builder.asynchronous { "1.0" } else { "2.0" },
    })
}

fn set_subsystem<T>(mut builder: ClassBuilder<T>, value: Box<str>) -> ClassBuilder<T>
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

        let identifier = Identifier::NameId("TESTJOBW".into(), "JOB00023".into());
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
}
