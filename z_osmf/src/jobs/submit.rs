use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JclSource {
    Data(JclData),
    Dataset(Box<str>),
    File(Box<str>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JclData {
    Binary(Bytes),
    Record(Bytes),
    Text(Box<str>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobEvent {
    Active,
    Complete,
    Ready,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restjobs/jobs{subsystem}")]
pub struct SubmitJobBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(optional, header = "X-IBM-Intrdr-Class", skip_setter)]
    message_class: Option<Box<str>>,
    #[endpoint(optional, header = "X-IBM-Intrdr-Lrecl")]
    record_length: Option<i32>,
    #[endpoint(optional, header = "X-IBM-User-Correlator")]
    user_correlator: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_symbols)]
    symbols: Option<HashMap<Box<str>, Box<str>>>,
    #[endpoint(builder_fn = build_jcl_source)]
    jcl_source: JclSource,
    #[endpoint(optional, header = "X-IBM-Notification-URL")]
    notification_url: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_notification_events)]
    notification_events: Option<Box<[JobEvent]>>,
    #[endpoint(optional, header = "X-IBM-Intrdr-File-Encoding")]
    encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> SubmitJobBuilder<T>
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
    mut request_builder: reqwest::RequestBuilder,
    builder: &SubmitJobBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder = match &builder.jcl_source {
        JclSource::Data(jcl_data) => match jcl_data {
            JclData::Binary(binary) => request_builder
                .header("X-IBM-Intrdr-Mode", "BINARY")
                .body(binary.clone()),
            JclData::Record(record) => request_builder
                .header("X-IBM-Intrdr-Mode", "RECORD")
                .body(record.clone()),
            JclData::Text(text) => request_builder.body(text.to_string()),
        },
        JclSource::Dataset(dataset) => request_builder.json(&Source {
            file: &format!("//'{}'", dataset),
        }),
        JclSource::File(file) => request_builder.json(&Source { file }),
    };

    request_builder
}

fn build_notification_events<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &SubmitJobBuilder<T>,
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

            let header_value = serde_json::to_string(&NotificationOptions { events }).unwrap();
            request_builder = request_builder.header("X-IBM-Notification-Options", header_value);
        }
    }

    request_builder
}

fn build_symbols<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &SubmitJobBuilder<T>,
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

fn set_subsystem<T>(mut builder: SubmitJobBuilder<T>, value: Box<str>) -> SubmitJobBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("/-{}", value).into();

    builder
}
