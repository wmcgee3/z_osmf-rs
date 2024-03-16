use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

use super::{Enqueue, MigratedRecall};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Write {
    etag: Box<str>,
    transaction_id: Box<str>,
}

impl TryFromResponse for Write {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let etag = get_etag(&value)?.ok_or(Error::Etag)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(Write {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct WriteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

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
    obtain_enq: Option<Enqueue>,
    #[endpoint(optional, header = "X-IBM-Session-Ref")]
    session_ref: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_release_enq)]
    release_enq: bool,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> WriteBuilder<T>
where
    T: TryFromResponse,
{
    pub fn binary<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        WriteBuilder {
            data: Some(Data::Binary(data.into())),
            ..self
        }
    }

    pub fn record<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        WriteBuilder {
            data: Some(Data::Record(data.into())),
            ..self
        }
    }

    pub fn text<S>(self, data: S) -> Self
    where
        S: ToString,
    {
        WriteBuilder {
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
    request_builder: reqwest::RequestBuilder,
    builder: &WriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let WriteBuilder {
        data,
        encoding,
        crlf_newlines,
        ..
    } = builder;

    match data {
        Some(Data::Binary(binary)) => request_builder
            .header("X-IBM-Data-Type", "binary")
            .body(binary.clone()),
        Some(Data::Record(record)) => request_builder
            .header("X-IBM-Data-Type", "record")
            .body(record.clone()),
        Some(Data::Text(text)) => match (encoding, crlf_newlines) {
            (Some(encoding), true) => request_builder.header(
                "X-IBM-Data-Type",
                format!("text;fileEncoding={};crlf=true", encoding),
            ),
            (Some(encoding), false) => {
                request_builder.header("X-IBM-Data-Type", format!("text;fileEncoding={}", encoding))
            }
            (None, true) => request_builder.header("X-IBM-Data-Type", "text;crlf=true"),
            (_, _) => request_builder,
        }
        .body(text.clone()),
        None => request_builder,
    }
}

fn build_release_enq<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.release_enq {
        true => request_builder.header("X-IBM-Release-ENQ", "true"),
        false => request_builder,
    }
}

fn set_member<T>(mut builder: WriteBuilder<T>, value: Box<str>) -> WriteBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<T>(mut builder: WriteBuilder<T>, value: Box<str>) -> WriteBuilder<T>
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
            .core
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
