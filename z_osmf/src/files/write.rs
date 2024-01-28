use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use z_osmf_macros::{Endpoint, Getters};

use crate::error::Error;
use crate::restfiles::{Binary, DataType, Text};
use crate::utils::{get_etag, get_transaction_id};

#[derive(Clone, Debug, Getters)]
pub struct FileWrite {
    etag: Box<str>,
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for FileWrite {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let etag = get_etag(&value)?.ok_or(Error::MissingEtag)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileWrite {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_builder)]
    crlf_newlines: bool,
    #[endpoint(optional, skip_setter, builder_fn = "build_data")]
    data: Option<D>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type: DataType,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, header = "If-Match")]
    if_match: Option<Box<str>>,
}

impl<D, T> FileWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    pub fn binary<B>(self, data: B) -> FileWriteBuilder<Bytes, Binary>
    where
        B: Into<Bytes>,
    {
        FileWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            crlf_newlines: self.crlf_newlines,
            data: Some(data.into()),
            data_type: self.data_type,
            data_type_marker: PhantomData,
            encoding: self.encoding,
            if_match: self.if_match,
        }
    }

    pub fn text<S>(self, data: S) -> FileWriteBuilder<String, Text>
    where
        S: ToString,
    {
        FileWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            path: self.path,
            crlf_newlines: self.crlf_newlines,
            data: Some(data.to_string()),
            data_type: self.data_type,
            data_type_marker: PhantomData,
            encoding: self.encoding,
            if_match: self.if_match,
        }
    }

    pub async fn build(self) -> Result<FileWrite, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

fn build_data<D, T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileWriteBuilder<D, T>,
) -> reqwest::RequestBuilder
where
    D: Into<reqwest::Body> + Clone,
{
    let key = "X-IBM-Data-Type";
    let FileWriteBuilder {
        crlf_newlines,
        data,
        data_type,
        encoding,
        ..
    } = builder;

    request_builder = match (data_type, encoding, crlf_newlines) {
        (&DataType::Text, encoding, crlf) => request_builder.header(
            key,
            format!(
                "text{}{}",
                if let Some(encoding) = encoding {
                    format!(";fileEncoding={}", encoding)
                } else {
                    "".to_string()
                },
                if *crlf { ";crlf=true" } else { "" }
            ),
        ),
        (data_type, _, _) => request_builder.header(key, format!("{}", data_type)),
    };
    if let Some(value) = data {
        request_builder = request_builder.body(value.clone());
    }

    request_builder
}
