use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::get_subsystem;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JclData {
    Binary(Bytes),
    Record(Bytes),
    Text(String),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobSource {
    Dataset(String),
    File(String),
    Jcl(JclData),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobNotificationEvent {
    Active,
    Complete,
    Ready,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobRecordFormat {
    Fixed,
    Variable,
}

impl From<JobRecordFormat> for reqwest::header::HeaderValue {
    fn from(value: JobRecordFormat) -> Self {
        match value {
            JobRecordFormat::Fixed => "F",
            JobRecordFormat::Variable => "V",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}")]
pub struct JobSubmitBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Arc<str>>,
    #[endpoint(header = "X-IBM-Intrdr-Class", skip_setter)]
    message_class: Option<Arc<str>>,
    #[endpoint(header = "X-IBM-Intrdr-Recfm")]
    record_format: Option<JobRecordFormat>,
    #[endpoint(header = "X-IBM-Intrdr-Lrecl")]
    record_length: Option<i32>,
    #[endpoint(header = "X-IBM-User-Correlator")]
    user_correlator: Option<Arc<str>>,
    #[endpoint(builder_fn = build_symbols)]
    symbols: Option<HashMap<Arc<str>, Arc<str>>>,
    #[endpoint(builder_fn = build_jcl_source)]
    jcl_source: JobSource,
    #[endpoint(header = "X-IBM-Notification-URL")]
    notification_url: Option<Arc<str>>,
    #[endpoint(builder_fn = build_notification_events)]
    notification_events: Option<Arc<[JobNotificationEvent]>>,
    #[endpoint(header = "X-IBM-Intrdr-File-Encoding")]
    encoding: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

impl<T> JobSubmitBuilder<T>
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
struct Source<'a> {
    file: &'a str,
}

fn build_jcl_source<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &JobSubmitBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match &builder.jcl_source {
        JobSource::Dataset(dataset) => request_builder
            .header("Content-Type", "application/json")
            .json(&Source {
                file: &format!("//'{}'", dataset),
            }),
        JobSource::File(file) => request_builder
            .header("Content-Type", "application/json")
            .json(&Source { file }),
        JobSource::Jcl(JclData::Binary(binary)) => request_builder
            .header("Content-Type", "application/octet-stream")
            .header("X-IBM-Intrdr-Mode", "BINARY")
            .body(binary.clone()),
        JobSource::Jcl(JclData::Record(record)) => request_builder
            .header("Content-Type", "application/octet-stream")
            .header("X-IBM-Intrdr-Mode", "RECORD")
            .body(record.clone()),
        JobSource::Jcl(JclData::Text(text)) => request_builder
            .header("Content-Type", "text/plain")
            .header("X-IBM-Intrdr-Mode", "TEXT")
            .body(text.to_string()),
    }
}

fn build_notification_events<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobSubmitBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if let Some(events) = &builder.notification_events {
        if !events.is_empty() {
            let mut events: Vec<&'static str> = events
                .iter()
                .map(|e| match e {
                    JobNotificationEvent::Active => "active",
                    JobNotificationEvent::Complete => "complete",
                    JobNotificationEvent::Ready => "ready",
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

fn build_subsystem<T>(builder: &JobSubmitBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}

fn build_symbols<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &JobSubmitBuilder<T>,
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
            .submit(JobSource::Jcl(JclData::Text(jcl.into())))
            .message_class('A')
            .record_format(JobRecordFormat::Fixed)
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
            .submit(JobSource::Jcl(JclData::Text(jcl.into())))
            .notification_events([JobNotificationEvent::Active, JobNotificationEvent::Ready])
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_data));

        assert_eq!(
            manual_request.body().unwrap().as_bytes(),
            job_data.body().unwrap().as_bytes()
        )
    }
}
