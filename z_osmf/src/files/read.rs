use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::utils::*;
use z_osmf_core::restfiles::data_type::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileRead<T> {
    data: T,
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs{path}")]
pub struct FileReadBuilder<T> {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, query = "search", builder_fn = "build_search")]
    search_pattern: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data_type")]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
}

impl<T> FileReadBuilder<T> {
    pub fn data_type_binary(self) -> FileReadBuilder<Binary> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_record(self) -> FileReadBuilder<Record> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }
    pub fn data_type_text(self) -> FileReadBuilder<Text> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }
}

impl FileReadBuilder<Binary> {
    pub async fn build(self) -> anyhow::Result<FileRead<Bytes>> {
        let response = self.get_response().await?;
        let (etag, transaction_id) = get_headers(&response)?;
        let data = response.bytes().await?;

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

impl FileReadBuilder<Record> {
    pub async fn build(self) -> anyhow::Result<FileRead<Bytes>> {
        let response = self.get_response().await?;
        let (etag, transaction_id) = get_headers(&response)?;
        let data = response.bytes().await?;

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

impl<'a> FileReadBuilder<Text> {
    pub async fn build(self) -> anyhow::Result<FileRead<Box<str>>> {
        let response = self.get_response().await?;
        let (etag, transaction_id) = get_headers(&response)?;
        let data = response.text().await?.into();

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

fn build_data_type<T>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &FileReadBuilder<T>,
) -> reqwest::RequestBuilder {
    let FileReadBuilder {
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

fn build_search<T>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &FileReadBuilder<T>,
) -> reqwest::RequestBuilder {
    let FileReadBuilder {
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

fn get_headers(response: &reqwest::Response) -> anyhow::Result<(Option<Box<str>>, Box<str>)> {
    Ok((get_etag(response)?, get_transaction_id(response)?))
}
