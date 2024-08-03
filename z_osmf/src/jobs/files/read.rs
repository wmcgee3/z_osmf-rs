pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::jobs::{get_subsystem, JobIdentifier};
use crate::{ClientCore, Result};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobFileId {
    Jcl,
    Id(i32),
}

impl std::fmt::Display for JobFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobFileId::Jcl => write!(f, "JCL"),
            JobFileId::Id(id) => write!(f, "{}", id),
        }
    }
}

impl From<i32> for JobFileId {
    fn from(value: i32) -> Self {
        JobFileId::Id(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobFileRead<T> {
    data: T,
}

impl JobFileRead<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for JobFileRead<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(JobFileRead {
            data: value.text().await?.into(),
        })
    }
}

impl JobFileRead<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for JobFileRead<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(JobFileRead {
            data: value.bytes().await?,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}/files/{id}/records")]
pub struct JobFileReadBuilder<'a, T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: JobIdentifier<'a>,
    #[endpoint(path)]
    id: JobFileId,
    #[endpoint(header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(skip_setter, query = "mode")]
    data_type: Option<DataType>,
    #[endpoint(query = "fileEncoding")]
    encoding: Option<Box<str>>,
    #[endpoint(query = "search")]
    search: Option<Box<str>>,
    #[endpoint(query = "research")]
    search_regex: Option<Box<str>>,
    #[endpoint(builder_fn = build_search_case_sensitive)]
    search_case_sensitive: Option<bool>,
    #[endpoint(query = "maxreturnsize")]
    search_max_return: Option<i32>,

    target_type: PhantomData<T>,
}

impl<'a, U> JobFileReadBuilder<'a, JobFileRead<U>>
where
    JobFileRead<U>: TryFromResponse,
{
    pub fn binary(self) -> JobFileReadBuilder<'a, JobFileRead<Bytes>> {
        JobFileReadBuilder {
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

    pub fn record(self) -> JobFileReadBuilder<'a, JobFileRead<Bytes>> {
        JobFileReadBuilder {
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

    pub fn text(self) -> JobFileReadBuilder<'a, JobFileRead<Box<str>>> {
        JobFileReadBuilder {
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
    builder: &JobFileReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        Some(true) => request_builder.query(&["insensitive", "false"]),
        _ => request_builder,
    }
}

fn build_subsystem<T>(builder: &JobFileReadBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_subsystem(&builder.subsystem)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tests::*;

    use super::*;

    #[test]
    fn read_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOBJ/JOB00023/files/1/records")
            .build()
            .unwrap();

        let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00023");
        let file_id = JobFileId::Id(1);
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

        let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00023");
        let file_id = JobFileId::Id(8);
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

        let identifier = JobIdentifier::NameId("TESTJOBJ", "JOB00060");
        let file_id = JobFileId::Jcl;

        let job_file = zosmf
            .jobs()
            .read_file(identifier, file_id)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_file))
    }
}
