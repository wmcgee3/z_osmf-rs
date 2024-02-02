pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};

use super::{get_session_ref, DataType, MigratedRecall, ObtainEnq};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct ReadDataset<T> {
    #[getter(skip)]
    data: T,
    etag: Option<Box<str>>,
    session_ref: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl ReadDataset<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for ReadDataset<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.text().await?.into();

        Ok(ReadDataset {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl ReadDataset<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for ReadDataset<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = value.bytes().await?;

        Ok(ReadDataset {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl ReadDataset<Option<Box<str>>> {
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

impl TryFromResponse for ReadDataset<Option<Box<str>>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.text().await?.into())
        };

        Ok(ReadDataset {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl ReadDataset<Option<Bytes>> {
    pub fn data(&self) -> Option<&Bytes> {
        self.data.as_ref()
    }
}

impl TryFromResponse for ReadDataset<Option<Bytes>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, session_ref, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.bytes().await?)
        };

        Ok(ReadDataset {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct ReadDatasetBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = set_volume)]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, query = "search", builder_fn = build_search)]
    search_pattern: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, header = "If-None-Match", skip_setter)]
    if_none_match: Option<Box<str>>,
    #[endpoint(optional, skip_setter, builder_fn = build_data_type)]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_return_etag)]
    return_etag: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(optional, header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<ObtainEnq>,
    #[endpoint(optional, header = "X-IBM-Session-Ref")]
    session_ref: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_release_enq)]
    release_enq: bool,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<U> ReadDatasetBuilder<ReadDataset<U>>
where
    ReadDataset<U>: TryFromResponse,
    ReadDataset<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> ReadDatasetBuilder<ReadDataset<Bytes>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Binary),
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

    pub fn record(self) -> ReadDatasetBuilder<ReadDataset<Bytes>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Record),
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

    pub fn text(self) -> ReadDatasetBuilder<ReadDataset<Box<str>>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Text),
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

    pub fn if_none_match<E>(self, etag: E) -> ReadDatasetBuilder<ReadDataset<Option<U>>>
    where
        E: Into<Box<str>>,
    {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            if_none_match: Some(etag.into()),
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

impl<V> ReadDatasetBuilder<ReadDataset<Option<V>>>
where
    ReadDataset<Option<V>>: TryFromResponse,
{
    pub fn binary(self) -> ReadDatasetBuilder<ReadDataset<Option<Bytes>>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Binary),
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

    pub fn record(self) -> ReadDatasetBuilder<ReadDataset<Option<Bytes>>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Record),
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

    pub fn text(self) -> ReadDatasetBuilder<ReadDataset<Option<Box<str>>>> {
        ReadDatasetBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Text),
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
    dataset_read_builder: &ReadDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let ReadDatasetBuilder {
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
    mut request_builder: reqwest::RequestBuilder,
    builder: &ReadDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn build_return_etag<T>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &ReadDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if dataset_read_builder.return_etag {
        request_builder = request_builder.header("X-IBM-Return-Etag", "true");
    }

    request_builder
}

fn build_search<T>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &ReadDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let ReadDatasetBuilder {
        search_pattern,
        search_is_regex,
        search_case_sensitive,
        search_max_return,
        ..
    } = &dataset_read_builder;

    if let Some(search) = search_pattern {
        request_builder = request_builder.query(&[(
            if *search_is_regex {
                "research"
            } else {
                "search"
            },
            search,
        )]);
        if *search_case_sensitive {
            request_builder = request_builder.query(&[("insensitive", "false")]);
        }
        if let Some(max) = search_max_return {
            request_builder = request_builder.query(&[("maxreturnsize", max)]);
        }
    }

    request_builder
}

type H = (Option<Box<str>>, Option<Box<str>>, Box<str>);

fn get_headers(response: &reqwest::Response) -> Result<H, Error> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}

fn set_member<T>(
    mut dataset_read_builder: ReadDatasetBuilder<T>,
    value: Box<str>,
) -> ReadDatasetBuilder<T>
where
    T: TryFromResponse,
{
    dataset_read_builder.member = format!("({})", value).into();

    dataset_read_builder
}

fn set_volume<T>(
    mut dataset_read_builder: ReadDatasetBuilder<T>,
    value: Box<str>,
) -> ReadDatasetBuilder<T>
where
    T: TryFromResponse,
{
    dataset_read_builder.volume = format!("-({})/", value).into();

    dataset_read_builder
}
