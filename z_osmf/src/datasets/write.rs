use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};

use super::{MigratedRecall, ObtainEnq};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct WriteDataset {
    etag: Box<str>,
    transaction_id: Box<str>,
}

impl TryFromResponse for WriteDataset {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let etag = get_etag(&value)?.ok_or(Error::MissingEtag)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(WriteDataset {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct WriteDatasetBuilder<T>
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
    #[endpoint(optional, header = "If-Match")]
    if_match: Option<Box<str>>,
    #[endpoint(optional, skip_setter, builder_fn = build_data)]
    data: Option<Data>,
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
    #[endpoint(optional, builder_fn = build_release_enq)]
    release_enq: bool,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> WriteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    pub fn binary<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        WriteDatasetBuilder {
            data: Some(Data::Binary(data.into())),
            ..self
        }
    }

    pub fn record<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        WriteDatasetBuilder {
            data: Some(Data::Record(data.into())),
            ..self
        }
    }

    pub fn text<S>(self, data: S) -> Self
    where
        S: ToString,
    {
        WriteDatasetBuilder {
            data: Some(Data::Text(data.to_string())),
            ..self
        }
    }
}

#[derive(Clone, Debug)]
enum Data {
    Binary(Bytes),
    Record(Bytes),
    Text(String),
}

fn build_data<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &WriteDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let key = "X-IBM-Data-Type";
    let WriteDatasetBuilder {
        data,
        encoding,
        crlf_newlines,
        ..
    } = builder;

    request_builder = match (data, encoding, crlf_newlines) {
        (Some(Data::Binary(_)), _, _) => request_builder.header(key, "binary"),
        (Some(Data::Record(_)), _, _) => request_builder.header(key, "record"),
        (Some(Data::Text(_)), encoding, crlf) if encoding.is_some() || *crlf => request_builder
            .header(
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
        _ => request_builder,
    };
    request_builder = match data {
        Some(Data::Binary(binary)) => request_builder.body(binary.clone()),
        Some(Data::Record(record)) => request_builder.body(record.clone()),
        Some(Data::Text(text)) => request_builder.body(text.clone()),
        None => request_builder,
    };

    request_builder
}

fn build_release_enq<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &WriteDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn set_member<T>(mut builder: WriteDatasetBuilder<T>, value: Box<str>) -> WriteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<T>(mut builder: WriteDatasetBuilder<T>, value: Box<str>) -> WriteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.volume = format!("-({})/", value).into();

    builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let string_data = "here is some text!";

        let manual_request = zosmf
            .client
            .put("https://test.com/zosmf/restfiles/ds/SYS1.PARMLIB(SMFPRM00)")
            .header("If-Match", "B5C6454F783590AA8EC15BD88E29EA63")
            .body(string_data)
            .build()
            .unwrap();

        let write_dataset = zosmf
            .datasets()
            .write("SYS1.PARMLIB")
            .member("SMFPRM00")
            .if_match("B5C6454F783590AA8EC15BD88E29EA63")
            .text(string_data)
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", write_dataset)
        );

        assert_eq!(
            manual_request.body().unwrap().as_bytes().unwrap(),
            write_dataset.body().unwrap().as_bytes().unwrap()
        )
    }
}
