use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileWriteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Arc<str>,

    #[endpoint(skip_builder)]
    crlf_newlines: Option<bool>,
    #[endpoint(skip_setter, builder_fn = build_data)]
    data: Option<Data>,
    #[endpoint(skip_builder)]
    encoding: Option<Arc<str>>,
    #[endpoint(header = "If-Match")]
    if_match: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

impl<T> FileWriteBuilder<T>
where
    T: TryFromResponse,
{
    pub fn binary<B>(mut self, data: B) -> Self
    where
        B: Into<Bytes>,
    {
        self.data = Some(Data::Binary(data.into()));

        self
    }

    pub fn text<B>(mut self, data: B) -> Self
    where
        B: std::fmt::Display,
    {
        self.data = Some(Data::Text(data.to_string().into()));

        self
    }
}

fn build_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileWriteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let FileWriteBuilder {
        crlf_newlines,
        data,
        encoding,
        ..
    } = builder;

    match data {
        Some(Data::Binary(binary)) => request_builder
            .body(binary.clone())
            .header("X-IBM-Data-Type", "binary"),
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
        .body(text.to_string()),
        _ => request_builder,
    }
}

#[derive(Clone, Debug)]
enum Data {
    Binary(Bytes),
    Text(Arc<str>),
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
