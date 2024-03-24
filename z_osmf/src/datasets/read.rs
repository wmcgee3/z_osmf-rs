pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

use super::{get_member, get_session_ref, get_volume, DatasetDataType, Enqueue, MigratedRecall};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Read<T> {
    #[getter(skip)]
    data: T,
    etag: Option<Box<str>>,
    session_ref: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl Read<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for Read<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.text().await?.into();

        Ok(Read {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl Read<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for Read<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.bytes().await?;

        Ok(Read {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl Read<Option<Box<str>>> {
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

impl TryFromResponse for Read<Option<Box<str>>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.text().await?.into())
        };

        Ok(Read {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl Read<Option<Bytes>> {
    pub fn data(&self) -> Option<&Bytes> {
        self.data.as_ref()
    }
}

impl TryFromResponse for Read<Option<Bytes>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.bytes().await?)
        };

        Ok(Read {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds{volume}/{dataset_name}{member}")]
pub struct ReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Box<str>>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Box<str>>,
    #[endpoint(query = "search")]
    search: Option<Box<str>>,
    #[endpoint(query = "research")]
    regex_search: Option<Box<str>>,
    #[endpoint(skip_builder)]
    search_is_regex: Option<bool>,
    #[endpoint(builder_fn = build_search_case_sensitive)]
    search_case_sensitive: Option<bool>,
    #[endpoint(query = "maxreturnsize")]
    search_max_return: Option<i32>,
    #[endpoint(header = "If-None-Match", skip_setter)]
    if_none_match: Option<Box<str>>,
    #[endpoint(skip_setter, builder_fn = build_data_type)]
    data_type: Option<DatasetDataType>,
    #[endpoint(skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(builder_fn = build_return_etag)]
    return_etag: Option<bool>,
    #[endpoint(header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<Enqueue>,
    #[endpoint(header = "X-IBM-Session-Ref")]
    session_ref: Option<Box<str>>,
    #[endpoint(builder_fn = build_release_enq)]
    release_enq: Option<bool>,
    #[endpoint(header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    target_type: PhantomData<T>,
}

impl<U> ReadBuilder<Read<U>>
where
    Read<U>: TryFromResponse,
    Read<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> ReadBuilder<Read<Bytes>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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

    pub fn record(self) -> ReadBuilder<Read<Bytes>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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

    pub fn text(self) -> ReadBuilder<Read<Box<str>>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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

    pub fn if_none_match<E>(self, etag: E) -> ReadBuilder<Read<Option<U>>>
    where
        E: std::fmt::Display,
    {
        ReadBuilder {
            core: self.core,
            dataset_name: self.dataset_name,
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

impl<V> ReadBuilder<Read<Option<V>>>
where
    Read<Option<V>>: TryFromResponse,
{
    pub fn binary(self) -> ReadBuilder<Read<Option<Bytes>>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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

    pub fn record(self) -> ReadBuilder<Read<Option<Bytes>>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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

    pub fn text(self) -> ReadBuilder<Read<Option<Box<str>>>> {
        ReadBuilder {
            core: self.core,
            search: self.search,
            regex_search: self.regex_search,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
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
    dataset_read_builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let ReadBuilder {
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
    builder: &ReadBuilder<T>,
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
    builder: &ReadBuilder<T>,
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
    builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        Some(true) => request_builder.query(&[("insensitive", "false")]),
        _ => request_builder,
    }
}

type H = (Option<Box<str>>, Option<Box<str>>, Box<str>);

fn get_headers(response: &reqwest::Response) -> Result<H, Error> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}

fn build_member<T>(builder: &ReadBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}

fn build_volume<T>(builder: &ReadBuilder<T>) -> String
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
