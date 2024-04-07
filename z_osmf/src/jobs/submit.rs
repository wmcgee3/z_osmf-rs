use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::get_subsystem;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Jcl {
    Binary(Bytes),
    Dataset(String),
    File(String),
    Record(Bytes),
    Text(String),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum JobEvent {
    Active,
    Complete,
    Ready,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum RecordFormat {
    Fixed,
    Variable,
}

impl From<RecordFormat> for reqwest::header::HeaderValue {
    fn from(value: RecordFormat) -> Self {
        match value {
            RecordFormat::Fixed => "F",
            RecordFormat::Variable => "V",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}")]
pub struct SubmitBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(header = "X-IBM-Intrdr-Class", skip_setter)]
    message_class: Option<Box<str>>,
    #[endpoint(header = "X-IBM-Intrdr-Recfm")]
    record_format: Option<RecordFormat>,
    #[endpoint(header = "X-IBM-Intrdr-Lrecl")]
    record_length: Option<i32>,
    #[endpoint(header = "X-IBM-User-Correlator")]
    user_correlator: Option<Box<str>>,
    #[endpoint(builder_fn = build_symbols)]
    symbols: Option<HashMap<Box<str>, Box<str>>>,
    #[endpoint(builder_fn = build_jcl_source)]
    jcl_source: Jcl,
    #[endpoint(header = "X-IBM-Notification-URL")]
    notification_url: Option<Box<str>>,
    #[endpoint(builder_fn = build_notification_events)]
    notification_events: Option<Box<[JobEvent]>>,
    #[endpoint(header = "X-IBM-Intrdr-File-Encoding")]
    encoding: Option<Box<str>>,

    target_type: PhantomData<T>,
}

impl<T> SubmitBuilder<T>
where
    T: TryFromResponse,
{
    pub fn message_class<C>(mut self, value: C) -> Self
    where
        C: Into<char>,
    {
        self.message_class = Some(value.into().to_string().into());

        self
    }
}

#[derive(Serialize)]
struct NotificationOptions {
    events: Vec<&'static str>,
}

#[derive(Serialize)]
struct Source<'a> {
    file: &'a str,
}

fn build_jcl_source<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &SubmitBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match &builder.jcl_source {
        Jcl::Binary(binary) => request_builder
            .header("Content-Type", "application/octet-stream")
            .header("X-IBM-Intrdr-Mode", "BINARY")
            .body(binary.clone()),
        Jcl::Dataset(dataset) => request_builder
            .header("Content-Type", "application/json")
            .json(&Source {
                file: &format!("//'{}'", dataset),
            }),
        Jcl::File(file) => request_builder
            .header("Content-Type", "application/json")
            .json(&Source { file }),
        Jcl::Record(record) => request_builder
            .header("Content-Type", "application/octet-stream")
            .header("X-IBM-Intrdr-Mode", "RECORD")
            .body(record.clone()),
        Jcl::Text(text) => request_builder
            .header("Content-Type", "text/plain")
            .header("X-IBM-Intrdr-Mode", "TEXT")
            .body(text.to_string()),
    }
}

fn build_notification_events<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &SubmitBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if let Some(events) = &builder.notification_events {
        if !events.is_empty() {
            let mut events: Vec<&'static str> = events
                .iter()
                .map(|e| match e {
                    JobEvent::Active => "active",
                    JobEvent::Complete => "complete",
                    JobEvent::Ready => "ready",
                })
                .collect();
            events.sort_unstable();
            events.dedup();

            let header_value = format!(
                r#"{{"events": [{}]}}"#,
                events
                    .iter()
                    .map(|e| format!(r#""{}""#, e))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            request_builder = request_builder.header("X-IBM-Notification-Options", header_value);
        }
    }

    request_builder
}

fn build_subsystem<T>(builder: &SubmitBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}

fn build_symbols<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &SubmitBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if let Some(symbols) = &builder.symbols {
        for (name, value) in symbols.iter() {
            request_builder =
                request_builder.header(format!("X-IBM-JCL-Symbol-{}", name), value.as_ref());
        }
    }

    request_builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    use super::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
        // EXEC PGM=IEFBR14
        "#;

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restjobs/jobs")
            .header("X-IBM-Intrdr-Class", "A")
            .header("X-IBM-Intrdr-Recfm", "F")
            .header("X-IBM-Intrdr-Lrecl", "80")
            .header("Content-Type", "text/plain")
            .header("X-IBM-Intrdr-Mode", "TEXT")
            .body(jcl.to_string())
            .build()
            .unwrap();

        let job_data = zosmf
            .jobs()
            .submit(Jcl::Text(jcl.into()))
            .message_class('A')
            .record_format(RecordFormat::Fixed)
            .record_length(80)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_data));

        assert_eq!(
            manual_request.body().unwrap().as_bytes(),
            job_data.body().unwrap().as_bytes()
        )
    }

    #[test]
    fn notification_events() {
        let zosmf = get_zosmf();

        let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
        // EXEC PGM=IEFBR14
        "#;

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restjobs/jobs")
            .header("Content-Type", "text/plain")
            .header("X-IBM-Intrdr-Mode", "TEXT")
            .header(
                "X-IBM-Notification-Options",
                r#"{"events": ["active", "ready"]}"#,
            )
            .body(jcl.to_string())
            .build()
            .unwrap();

        let job_data = zosmf
            .jobs()
            .submit(Jcl::Text(jcl.into()))
            .notification_events([JobEvent::Active, JobEvent::Ready])
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_data));

        assert_eq!(
            manual_request.body().unwrap().as_bytes(),
            job_data.body().unwrap().as_bytes()
        )
    }
}
