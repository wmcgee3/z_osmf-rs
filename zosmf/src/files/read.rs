use std::marker::PhantomData;

use bytes::Bytes;
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use zosmf_macros::{Endpoint, Getters};

use crate::data_type::{Binary, BytesDataType, DataType, Record, Text};
use crate::utils::{get_etag, get_transaction_id};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileRead<T> {
    data: T,
    etag: Option<String>,
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs{file_path}")]
pub struct FileReadBuilder<'a, T> {
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    file_path: String,

    #[endpoint(optional, skip_setter, builder_fn = "build_data_type")]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<String>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
}

impl<'a, T> FileReadBuilder<'a, T> {
    pub fn data_type_binary(self) -> FileReadBuilder<'a, Binary> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            file_path: self.file_path,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_record(self) -> FileReadBuilder<'a, Record> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            file_path: self.file_path,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }
    pub fn data_type_text(self) -> FileReadBuilder<'a, Text> {
        FileReadBuilder {
            base_url: self.base_url,
            client: self.client,
            file_path: self.file_path,
            data_type: self.data_type,
            encoding: self.encoding,
            data_type_marker: PhantomData,
        }
    }
}

impl<'a> FileReadBuilder<'a, Text> {
    pub async fn build(self) -> anyhow::Result<FileRead<String>> {
        let response = self.get_response().await?;
        let (etag, transaction_id) = get_headers(&response)?;
        let data = response.text().await?;

        Ok(FileRead {
            data,
            etag,
            transaction_id,
        })
    }
}

impl<'a, B> FileReadBuilder<'a, B>
where
    B: BytesDataType,
{
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

fn build_data_type<T>(
    request_builder: RequestBuilder,
    dataset_read_builder: &FileReadBuilder<T>,
) -> RequestBuilder {
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

fn get_headers(response: &Response) -> anyhow::Result<(Option<String>, String)> {
    Ok((get_etag(response)?, get_transaction_id(response)?))
}
