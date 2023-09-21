use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use zosmf_macros::{Endpoint, Getters};

use crate::data_type::*;
use crate::datasets::utils::*;
use crate::if_match::*;
use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetRead<T> {
    data: T,
    etag: Option<Box<str>>,
    session_ref: Option<Box<str>>,
    transaction_id: Box<str>,
}

#[derive(Clone, Debug)]
pub enum DatasetReadIfNoneMatch<T> {
    Modified(DatasetRead<T>),
    NotModified(DatasetReadNotModified),
}

#[derive(Clone, Debug, Getters)]
pub struct DatasetReadNotModified {
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetReadBuilder<T, I> {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: Box<str>,
    #[endpoint(optional, query = "search", builder_fn = "build_search")]
    search_pattern: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, header = "If-None-Match", skip_setter)]
    if_none_match: Option<Box<str>>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data_type")]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, builder_fn = "build_return_etag")]
    return_etag: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<ObtainEnq>,
    #[endpoint(optional, header = "X-IBM-Session-Ref")]
    session_ref: Option<Box<str>>,
    #[endpoint(optional, builder_fn = "build_release_enq")]
    release_enq: bool,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
    #[endpoint(optional, skip_setter, skip_builder)]
    if_none_match_marker: PhantomData<I>,
}

impl<T, I> DatasetReadBuilder<T, I> {
    pub fn data_type_binary(self) -> DatasetReadBuilder<Binary, I> {
        DatasetReadBuilder {
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
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
            if_none_match_marker: self.if_none_match_marker,
        }
    }

    pub fn data_type_record(self) -> DatasetReadBuilder<Record, I> {
        DatasetReadBuilder {
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
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
            if_none_match_marker: self.if_none_match_marker,
        }
    }

    pub fn data_type_text(self) -> DatasetReadBuilder<Text, I> {
        DatasetReadBuilder {
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
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
            if_none_match_marker: self.if_none_match_marker,
        }
    }

    pub fn if_none_match<V>(self, value: V) -> DatasetReadBuilder<T, Etag>
    where
        V: Into<Box<str>>,
    {
        DatasetReadBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            if_none_match: Some(value.into()),
            data_type: self.data_type,
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: self.data_type_marker,
            if_none_match_marker: PhantomData,
        }
    }
}

impl<'a> DatasetReadBuilder<Text, NoEtag> {
    pub async fn build(self) -> anyhow::Result<DatasetRead<String>> {
        let response = self.get_response().await?.error_for_status()?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.text().await?;

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl<'a> DatasetReadBuilder<Text, Etag> {
    pub async fn build(self) -> anyhow::Result<DatasetReadIfNoneMatch<String>> {
        let response = self.get_response().await?;

        if response.status() == 304 {
            let transaction_id = get_transaction_id(&response)?;

            return Ok(DatasetReadIfNoneMatch::NotModified(
                DatasetReadNotModified { transaction_id },
            ));
        }

        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.text().await?;

        Ok(DatasetReadIfNoneMatch::Modified(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        }))
    }
}

impl<B> DatasetReadBuilder<B, NoEtag>
where
    B: BytesDataType,
{
    pub async fn build(self) -> anyhow::Result<DatasetRead<Bytes>> {
        let response = self.get_response().await?.error_for_status()?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.bytes().await?;

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl<B> DatasetReadBuilder<B, Etag>
where
    B: BytesDataType,
{
    pub async fn build(self) -> anyhow::Result<DatasetReadIfNoneMatch<Bytes>> {
        let response = self.get_response().await?;
        if response.status() == 304 {
            let transaction_id = get_transaction_id(&response)?;

            return Ok(DatasetReadIfNoneMatch::NotModified(
                DatasetReadNotModified { transaction_id },
            ));
        }
        let response = response.error_for_status()?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.bytes().await?;

        Ok(DatasetReadIfNoneMatch::Modified(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        }))
    }
}

fn set_member<T, I>(
    mut dataset_read_builder: DatasetReadBuilder<T, I>,
    value: Box<str>,
) -> DatasetReadBuilder<T, I> {
    dataset_read_builder.member = format!("({})", value).into();

    dataset_read_builder
}

fn set_volume<T, I>(
    mut dataset_read_builder: DatasetReadBuilder<T, I>,
    value: Box<str>,
) -> DatasetReadBuilder<T, I> {
    dataset_read_builder.volume = format!("-({})/", value).into();

    dataset_read_builder
}

fn build_search<T, I>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T, I>,
) -> reqwest::RequestBuilder {
    let DatasetReadBuilder {
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

fn build_data_type<T, I>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T, I>,
) -> reqwest::RequestBuilder {
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

fn build_release_enq<T, I>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &DatasetReadBuilder<T, I>,
) -> reqwest::RequestBuilder {
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn build_return_etag<T, I>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T, I>,
) -> reqwest::RequestBuilder {
    if dataset_read_builder.return_etag {
        request_builder = request_builder.header("X-IBM-Return-Etag", "true");
    }

    request_builder
}

type H = (Option<Box<str>>, Option<Box<str>>, Box<str>);

fn get_headers(response: &reqwest::Response) -> anyhow::Result<H> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}
