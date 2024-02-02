use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};

use super::JobIdentifier;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct ListJobFiles {
    items: Box<[JobFile]>,
}

impl TryFromResponse for ListJobFiles {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(ListJobFiles {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFile {
    #[serde(rename = "jobname")]
    job_name: Box<str>,
    #[serde(rename = "recfm")]
    record_format: Box<str>,
    byte_count: i32,
    record_count: i32,
    job_correlator: Option<Box<str>>,
    class: Box<str>,
    #[serde(rename = "jobid")]
    job_id: Box<str>,
    id: i32,
    #[serde(rename = "ddname")]
    dd_name: Box<str>,
    records_url: Box<str>,
    #[serde(rename = "lrecl")]
    record_length: i32,
    subsystem: Box<str>,
    #[serde(rename = "stepname")]
    step_name: Option<Box<str>>,
    #[serde(rename = "procstep")]
    proc_step: Option<Box<str>>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}/files")]
pub struct ListJobFilesBuilder<T>
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

fn set_subsystem<T>(mut builder: ListJobFilesBuilder<T>, value: Box<str>) -> ListJobFilesBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}
