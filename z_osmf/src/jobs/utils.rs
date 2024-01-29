use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub exec_system: Box<str>,
    pub exec_member: Box<str>,
    pub exec_submitted: Box<str>,
    pub exec_ended: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobExecData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecStepData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub step_data: Vec<StepData>,
    pub exec_system: Box<str>,
    pub exec_member: Box<str>,
    pub exec_submitted: Box<str>,
    pub exec_ended: Box<str>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobExecStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobExecStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStepData {
    #[serde(rename = "jobid")]
    pub id: Box<str>,
    #[serde(rename = "jobname")]
    pub name: Box<str>,
    pub subsystem: Option<Box<str>>,
    pub owner: Box<str>,
    pub status: Option<Status>,
    pub job_type: Option<JobType>,
    pub class: Box<str>,
    #[serde(rename = "retcode")]
    pub return_code: Option<Box<str>>,
    pub url: Box<str>,
    pub files_url: Box<str>,
    pub job_correlator: Option<Box<str>>,
    pub phase: i32,
    pub phase_name: Box<str>,
    pub step_data: Vec<StepData>,
    pub reason_not_running: Option<Box<str>>,
}

impl JobStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.clone(), self.id.clone())
    }
}

impl TryFromResponse for JobStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug)]
pub enum JobIdentifier {
    NameId(Box<str>, Box<str>),
    Correlator(Box<str>),
}

impl std::fmt::Display for JobIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type JI = JobIdentifier;
        let items = match self {
            JI::Correlator(correlator) => vec![correlator.as_ref()],
            JI::NameId(name, id) => vec![name.as_ref(), id.as_ref()],
        };

        write!(f, "{}", items.join("/"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum JobType {
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "STC")]
    StartedTask,
    #[serde(rename = "TSU")]
    TsoUser,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Input,
    Output,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StepData {
    pub active: bool,
    #[serde(rename = "smfid")]
    pub smf_id: Box<str>,
    pub step_number: i32,
    #[serde(default)]
    pub selected_time: Option<Box<str>>,
    pub owner: Box<str>,
    pub program_name: Box<str>,
    pub step_name: Box<str>,
    #[serde(default)]
    pub path_name: Option<Box<str>>,
    #[serde(default)]
    pub substep_number: Option<i32>,
    #[serde(default)]
    pub end_time: Option<Box<str>>,
    pub proc_step_name: Box<str>,
    #[serde(default, rename = "completion")]
    pub completion_code: Option<Box<str>>,
    #[serde(default)]
    pub abend_reason_code: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFeedback {
    jobid: Box<str>,
    jobname: Box<str>,
    original_jobid: Option<Box<str>>,
    owner: Box<str>,
    member: Box<str>,
    sysname: Box<str>,
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
    class: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<Box<str>>,
}

impl ClassJson {
    pub(super) fn new<C>(class: C) -> Self
    where
        C: Into<Box<str>>,
    {
        ClassJson {
            class: class.into(),
            version: None,
        }
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

pub struct AsynchronousResponse;

impl TryFromResponse for AsynchronousResponse {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(AsynchronousResponse {})
    }
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

pub trait FeedbackJson {
    fn set_version<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Box<str>>;
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

impl FeedbackJson for RequestJson {
    fn set_version<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Box<str>>,
    {
        self.version = Some(value.into());

        self
    }
}
