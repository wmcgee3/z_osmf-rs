use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};

use super::JobIdentifier;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct JobsFileList {
    items: Box<[JobFile]>,
}

impl TryFromResponse for JobsFileList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(JobsFileList {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFile {
    jobname: Box<str>,
    recfm: Box<str>,
    byte_count: i32,
    record_count: i32,
    job_correlator: Option<Box<str>>,
    class: Box<str>,
    jobid: Box<str>,
    id: i32,
    ddname: Box<str>,
    records_url: Box<str>,
    lrecl: i32,
    subsystem: Box<str>,
    stepname: Option<Box<str>>,
    procstep: Option<Box<str>>,
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
