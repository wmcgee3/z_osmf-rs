pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::JobIdentifier;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
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
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}/files")]
pub struct JobFilesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path, setter_fn = set_job_files_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Read<T> {
    data: T,
}

impl Read<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for Read<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(Read {
            data: value.text().await?.into(),
        })
    }
}

impl Read<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for Read<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(Read {
            data: value.bytes().await?,
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Id {
    Jcl,
    Id(i32),
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Id::Jcl => write!(f, "JCL"),
            Id::Id(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}/files/{id}/records")]
pub struct ReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path, setter_fn = set_read_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(path)]
    id: Id,
    #[endpoint(optional, header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(optional, skip_setter, query = "mode")]
    data_type: Option<DataType>,
    #[endpoint(optional, query = "fileEncoding")]
    encoding: Option<Box<str>>,
    #[endpoint(optional, query = "search")]
    search: Option<Box<str>>,
    #[endpoint(optional, query = "research")]
    search_regex: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_search_case_sensitive)]
    search_case_sensitive: bool,
    #[endpoint(optional, query = "maxreturnsize")]
    search_max_return: Option<i32>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<U> ReadBuilder<Read<U>>
where
    Read<U>: TryFromResponse,
{
    pub fn binary(self) -> ReadBuilder<Read<Bytes>> {
        ReadBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }

    pub fn record(self) -> ReadBuilder<Read<Bytes>> {
        ReadBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Record),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> ReadBuilder<Read<Box<str>>> {
        ReadBuilder {
            core: self.core,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum DataType {
    Binary,
    Record,
    Text,
}

fn build_search_case_sensitive<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        true => request_builder.query(&["insensitive", "false"]),
        false => request_builder,
    }
}

fn set_job_files_subsystem<T>(mut builder: JobFilesBuilder<T>, value: Box<str>) -> JobFilesBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

fn set_read_subsystem<T>(mut builder: ReadBuilder<T>, value: Box<str>) -> ReadBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::*;

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

        let identifier = JobIdentifier::NameId("TESTJOB1".into(), "JOB00023".into());
        let job_files = zosmf.jobs().list_files(identifier).get_request().unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_files))
    }


    #[test]
    fn read_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOBJ/JOB00023/files/1/records")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
        let file_id = Id::Id(1);
        let job_file = zosmf
            .jobs()
            .read_file(identifier, file_id)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_file))
    }

    #[test]
    fn read_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOBJ/JOB00023/files/8/records")
            .header("X-IBM-Record-Range", "0-249")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
        let file_id = Id::Id(8);
        let job_file = zosmf
            .jobs()
            .read_file(identifier, file_id)
            .record_range(RecordRange::from_str("0-249").unwrap())
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_file))
    }

    #[test]
    fn read_3() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOBJ/JOB00060/files/JCL/records")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00060".into());
        let file_id = Id::Jcl;

        let job_file = zosmf
            .jobs()
            .read_file(identifier, file_id)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_file))
    }
}
