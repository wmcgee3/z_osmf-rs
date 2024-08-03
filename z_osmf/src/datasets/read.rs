pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::restfiles::{get_etag, get_transaction_id};
use crate::{ClientCore, Result};

use super::{
    get_member, get_session_ref, get_volume, DatasetDataType, DatasetEnqueue, DatasetMigratedRecall,
};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DatasetRead<T> {
    #[getter(skip)]
    data: T,
    etag: Option<Arc<str>>,
    session_ref: Option<Arc<str>>,
    transaction_id: Arc<str>,
}

impl DatasetRead<Arc<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for DatasetRead<Arc<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.text().await?.into();

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl DatasetRead<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for DatasetRead<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.bytes().await?;

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl DatasetRead<Option<Arc<str>>> {
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

impl TryFromResponse for DatasetRead<Option<Arc<str>>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.text().await?.into())
        };

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl DatasetRead<Option<Bytes>> {
    pub fn data(&self) -> Option<&Bytes> {
        self.data.as_ref()
    }
}

impl TryFromResponse for DatasetRead<Option<Bytes>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.bytes().await?)
        };

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds{volume}/{dataset}{member}")]
pub struct DatasetReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    dataset: Arc<str>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Arc<str>>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Arc<str>>,
    #[endpoint(query = "search")]
    search: Option<Arc<str>>,
    #[endpoint(query = "research")]
    regex_search: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    search_is_regex: Option<bool>,
    #[endpoint(builder_fn = build_search_case_sensitive)]
    search_case_sensitive: Option<bool>,
    #[endpoint(query = "maxreturnsize")]
    search_max_return: Option<i32>,
    #[endpoint(header = "If-None-Match", skip_setter)]
    if_none_match: Option<Arc<str>>,
    #[endpoint(skip_setter, builder_fn = build_data_type)]
    data_type: Option<DatasetDataType>,
    #[endpoint(skip_builder)]
    encoding: Option<Arc<str>>,
    #[endpoint(builder_fn = build_return_etag)]
    return_etag: Option<bool>,
    #[endpoint(header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<DatasetMigratedRecall>,
    #[endpoint(header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<DatasetEnqueue>,
    #[endpoint(header = "X-IBM-Session-Ref")]
    session_ref: Option<Arc<str>>,
    #[endpoint(builder_fn = build_release_enq)]
    release_enq: Option<bool>,
    #[endpoint(header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

impl<U> DatasetReadBuilder<DatasetRead<U>>
where
    DatasetRead<U>: TryFromResponse,
    DatasetRead<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> DatasetReadBuilder<DatasetRead<Bytes>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Binary),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }

    pub fn record(self) -> DatasetReadBuilder<DatasetRead<Bytes>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Record),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> DatasetReadBuilder<DatasetRead<Arc<str>>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Text),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }

    pub fn if_none_match<E>(self, etag: E) -> DatasetReadBuilder<DatasetRead<Option<U>>>
    where
        E: std::fmt::Display,
    {
        DatasetReadBuilder {
            core: self.core,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            if_none_match: Some(etag.to_string().into()),
            data_type: self.data_type,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }
}

impl<V> DatasetReadBuilder<DatasetRead<Option<V>>>
where
    DatasetRead<Option<V>>: TryFromResponse,
{
    pub fn binary(self) -> DatasetReadBuilder<DatasetRead<Option<Bytes>>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Binary),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }

    pub fn record(self) -> DatasetReadBuilder<DatasetRead<Option<Bytes>>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Record),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> DatasetReadBuilder<DatasetRead<Option<Arc<str>>>> {
        DatasetReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset: self.dataset,
            volume: self.volume,
            member: self.member,
            data_type: Some(DatasetDataType::Text),
            if_none_match: self.if_none_match,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            record_range: self.record_range,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            target_type: PhantomData,
        }
    }
}

fn build_data_type<T>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let DatasetReadBuilder {
        data_type,
        encoding,
        ..
    } = &dataset_read_builder;

    let key = "X-IBM-Data-Type";

    match (data_type, encoding) {
        (Some(data_type), Some(encoding)) => {
            request_builder.header(key, format!("{};fileEncoding={}", data_type, encoding))
        }
        (Some(data_type), None) => request_builder.header(key, format!("{}", data_type)),
        (None, Some(encoding)) => {
            request_builder.header(key, format!("text;fileEncoding={}", encoding))
        }
        (None, None) => request_builder,
    }
}

fn build_release_enq<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.release_enq {
        Some(true) => request_builder.header("X-IBM-Release-ENQ", "true"),
        _ => request_builder,
    }
}

fn build_return_etag<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.return_etag {
        Some(true) => request_builder.header("X-IBM-Return-Etag", "true"),
        _ => request_builder,
    }
}

fn build_search_case_sensitive<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        Some(true) => request_builder.query(&[("insensitive", "false")]),
        _ => request_builder,
    }
}

type H = (Option<Arc<str>>, Option<Arc<str>>, Arc<str>);

fn get_headers(response: &reqwest::Response) -> Result<H> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}

fn build_member<T>(builder: &DatasetReadBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}

fn build_volume<T>(builder: &DatasetReadBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_volume(&builder.volume)
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/ds/SYS1.PARMLIB(SMFPRM00)")
            .build()
            .unwrap();

        let read_member = zosmf
            .datasets()
            .read("SYS1.PARMLIB")
            .member("SMFPRM00")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", read_member)
        );
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/ds/JIAHJ.REST.SRVMP")
            .build()
            .unwrap();

        let read_dataset = zosmf
            .datasets()
            .read("JIAHJ.REST.SRVMP")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", read_dataset)
        )
    }
}
