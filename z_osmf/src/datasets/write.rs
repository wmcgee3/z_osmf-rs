use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_core::error::Error;
use z_osmf_core::restfiles::data_type::*;
use z_osmf_macros::{Endpoint, Getters};

use crate::datasets::utils::*;
use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetWrite {
    etag: Box<str>,
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for DatasetWrite {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let etag = get_etag(&value)?.ok_or(Error::MissingEtag)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(DatasetWrite {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: Box<str>,
    #[endpoint(optional, header = "If-Match")]
    if_match: Option<Box<str>>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data")]
    data: Option<D>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type: DataType,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    crlf_newlines: bool,
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
}

impl<D, T> DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    pub fn binary<B>(self, data: B) -> DatasetWriteBuilder<Bytes, Binary>
    where
        B: Into<Bytes>,
    {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            if_match: self.if_match,
            data_type: DataType::Binary,
            data: Some(data.into()),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn record<B>(self, data: B) -> DatasetWriteBuilder<Bytes, Record>
    where
        B: Into<Bytes>,
    {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            if_match: self.if_match,
            data_type: DataType::Record,
            data: Some(data.into()),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn text<S>(self, data: S) -> DatasetWriteBuilder<String, Text>
    where
        S: ToString,
    {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            if_match: self.if_match,
            data_type: DataType::Text,
            data: Some(data.to_string()),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub async fn build(self) -> Result<DatasetWrite, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

fn build_data<D, T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &DatasetWriteBuilder<D, T>,
) -> reqwest::RequestBuilder
where
    D: Into<reqwest::Body> + Clone,
{
    let key = "X-IBM-Data-Type";
    let DatasetWriteBuilder {
        data_type,
        data,
        encoding,
        crlf_newlines,
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

fn build_release_enq<D, T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &DatasetWriteBuilder<D, T>,
) -> reqwest::RequestBuilder
where
    D: Into<reqwest::Body> + Clone,
{
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn set_member<D, T>(
    mut builder: DatasetWriteBuilder<D, T>,
    value: Box<str>,
) -> DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<D, T>(
    mut builder: DatasetWriteBuilder<D, T>,
    value: Box<str>,
) -> DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    builder.volume = format!("-({})/", value).into();

    builder
}
