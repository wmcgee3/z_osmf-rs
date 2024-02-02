use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;

use super::JobIdentifier;

pub struct AsynchronousResponse;

impl TryFromResponse for AsynchronousResponse {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(AsynchronousResponse {})
    }
}

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
    base_url: Arc<str>,
    client: reqwest::Client,

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
            base_url: self.base_url,
            client: self.client,
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
