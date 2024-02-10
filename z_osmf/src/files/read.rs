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

use super::FileDataType;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileRead<T> {
    #[getter(skip)]
    data: T,
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl FileRead<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
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

impl FileRead<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
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

impl FileRead<Option<Box<str>>> {
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

impl TryFromResponse for FileRead<Option<Box<str>>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.text().await?.into())
        };

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

impl FileRead<Option<Bytes>> {
    pub fn data(&self) -> Option<&Bytes> {
        self.data.as_ref()
    }
}

impl TryFromResponse for FileRead<Option<Bytes>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.bytes().await?)
        };

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs{path}")]
pub struct FileReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, query = "search", builder_fn = build_search)]
    search_pattern: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = build_data_type)]
    data_type: Option<FileDataType>,
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
    FileRead<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> FileReadBuilder<FileRead<Bytes>> {
        FileReadBuilder {
            core: self.core,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(FileDataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> FileReadBuilder<FileRead<Box<str>>> {
        FileReadBuilder {
            core: self.core,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(FileDataType::Text),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn if_none_match<E>(self, etag: E) -> FileReadBuilder<FileRead<Option<U>>>
    where
        E: Into<Box<str>>,
    {
        FileReadBuilder {
            core: self.core,
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

impl<U> FileReadBuilder<FileRead<Option<U>>>
where
    FileRead<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> FileReadBuilder<FileRead<Option<Bytes>>> {
        FileReadBuilder {
            core: self.core,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(FileDataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> FileReadBuilder<FileRead<Option<Box<str>>>> {
        FileReadBuilder {
            core: self.core,
            path: self.path,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(FileDataType::Text),
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

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/etc/inetd.conf")
            .build()
            .unwrap();

        let read_file = zosmf.files().read("/etc/inetd.conf").get_request().unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", read_file))
    }
}
