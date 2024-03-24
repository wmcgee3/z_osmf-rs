pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::jobs::{get_subsystem, Identifier};
use crate::ClientCore;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum DataType {
    Binary,
    Record,
    Text,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
pub enum FileId {
    Jcl,
    Id(i32),
}

impl std::fmt::Display for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileId::Jcl => write!(f, "JCL"),
            FileId::Id(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs{subsystem}/{identifier}/files/{id}/records")]
pub struct ReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_subsystem)]
    subsystem: Option<Box<str>>,
    #[endpoint(path)]
    identifier: Identifier,
    #[endpoint(path)]
    id: FileId,
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

fn build_search_case_sensitive<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        Some(true) => request_builder.query(&["insensitive", "false"]),
        _ => request_builder,
    }
}

fn build_subsystem<T>(builder: &ReadBuilder<T>) -> String
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
    fn job_files_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restjobs/jobs/TESTJOB1/JOB00023/files")
            .build()
            .unwrap();

        let identifier = Identifier::NameId("TESTJOB1".into(), "JOB00023".into());
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

        let identifier = Identifier::NameId("TESTJOBJ".into(), "JOB00023".into());
        let file_id = FileId::Id(1);
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

        let identifier = Identifier::NameId("TESTJOBJ".into(), "JOB00023".into());
        let file_id = FileId::Id(8);
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

        let identifier = Identifier::NameId("TESTJOBJ".into(), "JOB00060".into());
        let file_id = FileId::Jcl;

        let job_file = zosmf
            .jobs()
            .read_file(identifier, file_id)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", job_file))
    }
}
