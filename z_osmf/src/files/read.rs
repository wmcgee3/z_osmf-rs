use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::files::DataType;
use crate::restfiles::{get_etag, get_transaction_id};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileRead<T> {
    data: T,
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl TryFromResponse for FileRead<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = value.bytes().await?;

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

impl TryFromResponse for FileRead<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = value.text().await?.into();

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug)]
pub enum FileReadIfNoneMatch<T> {
    Modified(FileRead<T>),
    NotModified(FileReadNotModified),
}

impl TryFromResponse for FileReadIfNoneMatch<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        if value.status() == 304 {
            let transaction_id = get_transaction_id(&value)?;

            return Ok(FileReadIfNoneMatch::NotModified(FileReadNotModified {
                transaction_id,
            }));
        }

        let (etag, transaction_id) = get_headers(&value)?;
        let data = value.bytes().await?;

        Ok(FileReadIfNoneMatch::Modified(FileRead {
            data,
            etag,
            transaction_id,
        }))
    }
}

impl TryFromResponse for FileReadIfNoneMatch<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        if value.status() == 304 {
            let transaction_id = get_transaction_id(&value)?;

            return Ok(FileReadIfNoneMatch::NotModified(FileReadNotModified {
                transaction_id,
            }));
        }

        let (etag, transaction_id) = get_headers(&value)?;
        let data = value.text().await?.into();

        Ok(FileReadIfNoneMatch::Modified(FileRead {
            data,
            etag,
            transaction_id,
        }))
    }
}

#[derive(Clone, Debug, Getters)]
pub struct FileReadNotModified {
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs{path}")]
pub struct FileReadBuilder<T>
where
    T: TryFromResponse,
{
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
    #[endpoint(optional, header = "If-None-Match", skip_setter)]
    etag: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<U> FileReadBuilder<FileRead<U>>
where
    FileRead<U>: TryFromResponse,
    FileReadIfNoneMatch<U>: TryFromResponse,
{
    pub fn binary(self) -> FileReadBuilder<FileRead<Bytes>> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> FileReadBuilder<FileRead<Box<str>>> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn if_none_match<E>(self, etag: E) -> FileReadBuilder<FileReadIfNoneMatch<U>>
    where
        E: Into<Box<str>>,
    {
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
            etag: Some(etag.into()),
            target_type: PhantomData,
        }
    }
}

impl<U> FileReadBuilder<FileReadIfNoneMatch<U>>
where
    FileRead<U>: TryFromResponse,
    FileReadIfNoneMatch<U>: TryFromResponse,
{
    pub fn binary(self) -> FileReadBuilder<FileReadIfNoneMatch<Bytes>> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> FileReadBuilder<FileReadIfNoneMatch<Box<str>>> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }
}

fn build_data_type<T>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &FileReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
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
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
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

fn get_headers(response: &reqwest::Response) -> Result<(Option<Box<str>>, Box<str>), Error> {
    Ok((get_etag(response)?, get_transaction_id(response)?))
}
