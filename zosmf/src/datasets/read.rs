use std::marker::PhantomData;

use bytes::Bytes;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use zosmf_macros::{Endpoint, Getters};

use crate::utils::{get_etag, get_session_ref, get_transaction_id};

use super::utils::MigratedRecall;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetRead<T> {
    contents: T,
    etag: Option<String>,
    session_ref: Option<String>,
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetReadBuilder<'a, T> {
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    dataset_name: String,

    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: String,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: String,
    #[endpoint(optional, query = "search", builder_fn = "build_search")]
    search_pattern: Option<String>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data_type")]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<String>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
    #[endpoint(optional, builder_fn = "build_return_etag")]
    return_etag: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
}

impl<'a, T> DatasetReadBuilder<'a, T> {
    pub fn data_type_binary(self) -> DatasetReadBuilder<'a, Binary> {
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
            encoding: self.encoding,
            data_type_marker: PhantomData,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
        }
    }

    pub fn data_type_record(self) -> DatasetReadBuilder<'a, Record> {
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
            encoding: self.encoding,
            data_type_marker: PhantomData,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
        }
    }

    pub fn data_type_text(self) -> DatasetReadBuilder<'a, Text> {
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
            encoding: self.encoding,
            data_type_marker: PhantomData,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
        }
    }
}

impl<'a> DatasetReadBuilder<'a, Binary> {
    pub async fn build(self) -> anyhow::Result<DatasetRead<Bytes>> {
        let response = self.get_response().await?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let contents = response.bytes().await?;

        Ok(DatasetRead {
            contents,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl<'a> DatasetReadBuilder<'a, Record> {
    pub async fn build(self) -> anyhow::Result<DatasetRead<Bytes>> {
        let response = self.get_response().await?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let contents = response.bytes().await?;

        Ok(DatasetRead {
            contents,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl<'a> DatasetReadBuilder<'a, Text> {
    pub async fn build(self) -> anyhow::Result<DatasetRead<String>> {
        let response = self.get_response().await?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let contents = response.text().await?;

        Ok(DatasetRead {
            contents,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

pub struct Binary;
pub struct Record;
pub struct Text;

#[derive(Clone, Debug, PartialEq)]
enum DataType {
    Binary,
    Record,
    Text,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataType::Binary => "binary",
            DataType::Record => "record",
            DataType::Text => "text",
        };

        write!(f, "{}", s)
    }
}

fn set_member<T>(
    mut dataset_read_builder: DatasetReadBuilder<T>,
    value: String,
) -> DatasetReadBuilder<T> {
    dataset_read_builder.member = format!("({})", value);

    dataset_read_builder
}

fn set_volume<T>(
    mut dataset_read_builder: DatasetReadBuilder<T>,
    value: String,
) -> DatasetReadBuilder<T> {
    dataset_read_builder.volume = format!("-({})/", value);

    dataset_read_builder
}

fn build_search<T>(
    mut request_builder: RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> RequestBuilder {
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

fn build_data_type<T>(
    request_builder: RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> RequestBuilder {
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

fn build_return_etag<T>(
    mut request_builder: RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> RequestBuilder {
    if dataset_read_builder.return_etag {
        request_builder = request_builder.header("X-IBM-Return-Etag", "true");
    }

    request_builder
}

fn get_headers(response: &Response) -> anyhow::Result<(Option<String>, Option<String>, String)> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}
