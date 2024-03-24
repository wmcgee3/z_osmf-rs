pub use self::read::{FileId, Read, ReadBuilder, RecordRange};

mod read;

use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_subsystem, Identifier};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct JobFiles {
    items: Box<[JobFile]>,
}

impl TryFromResponse for JobFiles {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(JobFiles {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFile {
    #[serde(rename = "jobname")]
    job_name: Box<str>,
    #[serde(rename = "recfm")]
    record_format: Box<str>,
    #[getter(copy)]
    byte_count: i32,
    #[getter(copy)]
    record_count: i32,
    job_correlator: Option<Box<str>>,
    class: Box<str>,
    #[serde(rename = "jobid")]
    job_id: Box<str>,
    #[getter(copy)]
    id: i32,
    #[serde(rename = "ddname")]
    dd_name: Box<str>,
    records_url: Box<str>,
    #[getter(copy)]
    #[serde(rename = "lrecl")]
    record_length: i32,
    subsystem: Box<str>,
    #[serde(rename = "stepname")]
    step_name: Option<Box<str>>,
    #[serde(rename = "procstep")]
    proc_step: Option<Box<str>>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}/files")]
pub struct JobFilesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: Identifier,

    target_type: PhantomData<T>,
}

fn build_subsystem<T>(builder: &JobFilesBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}
