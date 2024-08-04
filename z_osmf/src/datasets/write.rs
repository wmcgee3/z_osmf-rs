use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_member, get_volume, DatasetEnqueue, DatasetMigratedRecall};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds{volume}/{dataset}{member}")]
pub struct DatasetWriteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    dataset: Arc<str>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Arc<str>>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Arc<str>>,
    #[endpoint(header = "If-Match")]
    if_match: Option<Arc<str>>,
    #[endpoint(skip_setter, builder_fn = build_data)]
    data: Option<Data>,
    #[endpoint(skip_builder)]
    encoding: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    crlf_newlines: Option<bool>,
    #[endpoint(header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<DatasetMigratedRecall>,
    #[endpoint(header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<DatasetEnqueue>,
    #[endpoint(header = "X-IBM-Session-Ref")]
    session_ref: Option<Arc<str>>,
    #[endpoint(builder_fn = build_release_enq)]
    release_enq: Option<bool>,
    #[endpoint(header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

impl<T> DatasetWriteBuilder<T>
where
    T: TryFromResponse,
{
    pub fn binary<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        DatasetWriteBuilder {
            data: Some(Data::Binary(data.into())),
            ..self
        }
    }

    pub fn record<B>(self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        DatasetWriteBuilder {
            data: Some(Data::Record(data.into())),
            ..self
        }
    }

    pub fn text<S>(self, data: S) -> Self
    where
        S: std::fmt::Display,
    {
        DatasetWriteBuilder {
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
    builder: &DatasetWriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let DatasetWriteBuilder {
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
            (Some(encoding), Some(true)) => request_builder.header(
                "X-IBM-Data-Type",
                format!("text;fileEncoding={};crlf=true", encoding),
            ),
            (Some(encoding), _) => {
                request_builder.header("X-IBM-Data-Type", format!("text;fileEncoding={}", encoding))
            }
            (None, Some(true)) => request_builder.header("X-IBM-Data-Type", "text;crlf=true"),
            _ => request_builder,
        }
        .body(text.clone()),
        None => request_builder,
    }
}

fn build_member<T>(builder: &DatasetWriteBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}

fn build_release_enq<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetWriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.release_enq {
        Some(true) => request_builder.header("X-IBM-Release-ENQ", "true"),
        _ => request_builder,
    }
}

fn build_volume<T>(builder: &DatasetWriteBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_volume(&builder.volume)
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
