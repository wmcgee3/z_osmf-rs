use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{FileTagLinks, FileTagType};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileSetTagBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    tag_type: Option<FileTagType>,
    #[endpoint(optional, skip_builder)]
    code_set: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    links: Option<FileTagLinks>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Debug, Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    action: &'static str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    tag_type: Option<FileTagType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codeset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<FileTagLinks>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileSetTagBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chtag",
        action: "set",
        tag_type: builder.tag_type,
        codeset: builder.code_set.as_deref(),
        links: builder.links,
        recursive: builder.recursive,
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use serde_json::{from_str, Value};

    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn maximal() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "chtag",
                "action": "set",
                "recursive": false,
                "type": "mixed",
                "codeset": "IBM-1047",
                "links": "suppress"
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .set_tag("/u/jiahj/testFile.txt")
            .tag_type(FileTagType::Mixed)
            .code_set("IBM-1047")
            .links(FileTagLinks::Suppress)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
