pub mod read;

use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::jobs::{get_subsystem, JobIdentifier};
use crate::{ClientCore, Result};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobFile {
    #[serde(rename = "jobname")]
    job_name: Arc<str>,
    #[serde(rename = "recfm")]
    record_format: Arc<str>,
    #[getter(copy)]
    byte_count: i32,
    #[getter(copy)]
    record_count: i32,
    job_correlator: Option<Arc<str>>,
    class: Arc<str>,
    #[serde(rename = "jobid")]
    job_id: Arc<str>,
    #[getter(copy)]
    id: i32,
    #[serde(rename = "ddname")]
    dd_name: Arc<str>,
    records_url: Arc<str>,
    #[getter(copy)]
    #[serde(rename = "lrecl")]
    record_length: i32,
    subsystem: Arc<str>,
    #[serde(rename = "stepname")]
    step_name: Option<Arc<str>>,
    #[serde(rename = "procstep")]
    proc_step: Option<Arc<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobFileList {
    items: Arc<[JobFile]>,
}

impl TryFromResponse for JobFileList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(JobFileList {
            items: value.json().await?,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}/files")]
pub struct JobFileListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Arc<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier,

    target_type: PhantomData<T>,
}

fn build_subsystem<T>(builder: &JobFileListBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}

#[cfg(test)]
mod tests {
    use crate::tests::get_zosmf;

    use super::*;

    #[test]
    fn job_files_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOB1/JOB00023/files")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOB1".to_string(), "JOB00023".to_string());
        let job_files = zosmf.jobs().list_files(identifier).get_request().unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_files))
    }
}
