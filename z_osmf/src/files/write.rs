use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileWrite {
    etag: Box<str>,
    transaction_id: Box<str>,
}

impl TryFromResponse for FileWrite {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let etag = get_etag(&value)?.ok_or(Error::Etag)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileWrite {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileWriteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_builder)]
    crlf_newlines: bool,
    #[endpoint(optional, skip_setter, builder_fn = build_data)]
    data: Option<Data>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, header = "If-Match")]
    if_match: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl FileWriteBuilder<FileWrite> {
    pub fn binary<B>(mut self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        self.data = Some(Data::Binary(data.into()));

        self
    }

    pub fn text<B>(mut self, data: B) -> Self
    where
        B: Into<Box<str>>,
    {
        self.data = Some(Data::Text(data.into()));

        self
    }
}

fn build_data<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileWriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let key = "X-IBM-Data-Type";
    let FileWriteBuilder {
        crlf_newlines,
        data,
        encoding,
        ..
    } = builder;

    request_builder = match (data, encoding, crlf_newlines) {
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
        (Some(Data::Binary(_)), _, _) => request_builder.header(key, "binary"),
        _ => request_builder,
    };

    match data {
        Some(Data::Binary(binary)) => request_builder.body(binary.clone()),
        Some(Data::Text(text)) => request_builder.body(text.to_string()),
        _ => request_builder,
    }
}

#[derive(Clone, Debug)]
enum Data {
    Binary(Bytes),
    Text(Box<str>),
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::tests::*;

    #[test]
    fn binary() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .header("x-ibm-data-type", "binary")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .write("/u/jiahj/testFile.txt")
            .binary(Bytes::from("some text"))
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request))
    }

    #[test]
    fn encoding() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .header("x-ibm-data-type", "text;fileEncoding=IBM-1047;crlf=true")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .write("/u/jiahj/testFile.txt")
            .text("some data")
            .crlf_newlines(true)
            .encoding("IBM-1047")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request))
    }

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let text_data = "here is some text!";

        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/etc/inetd.conf")
            .body(text_data)
            .build()
            .unwrap();

        let write_file = zosmf
            .files()
            .write("/etc/inetd.conf")
            .text(text_data)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", write_file))
    }
}
