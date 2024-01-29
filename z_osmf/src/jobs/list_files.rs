use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};

use super::JobIdentifier;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobsFileList {
    pub items: Box<[JobFile]>,
}

impl TryFromResponse for JobsFileList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(JobsFileList {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFile {
    pub jobname: Box<str>,
    pub recfm: Box<str>,
    pub byte_count: i32,
    pub record_count: i32,
    pub job_correlator: Option<Box<str>>,
    pub class: Box<str>,
    pub jobid: Box<str>,
    pub id: i32,
    pub ddname: Box<str>,
    pub records_url: Box<str>,
    pub lrecl: i32,
    pub subsystem: Box<str>,
    pub stepname: Option<Box<str>>,
    pub procstep: Option<Box<str>>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}/files")]
pub struct JobsFileListBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn set_subsystem<T>(mut builder: JobsFileListBuilder<T>, value: Box<str>) -> JobsFileListBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}
