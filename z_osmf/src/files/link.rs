use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{target_path}")]
pub struct LinkBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    source_path: Box<str>,
    #[endpoint(path)]
    target_path: Box<str>,
    #[endpoint(skip_builder)]
    link_type: LinkType,
    #[endpoint(skip_builder)]
    recursive: Option<bool>,
    #[endpoint(skip_builder)]
    force: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkType {
    External,
    Symbol,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    from: &'a str,
    #[serde(rename = "type")]
    link_type: LinkType,
    recursive: bool,
    force: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &LinkBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "link",
        from: &builder.source_path,
        link_type: builder.link_type,
        recursive: builder.recursive == Some(true),
        force: builder.force == Some(true),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::*;

    use super::LinkType;

    #[test]
    fn maximal() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "link",
                "from": "/u/jiahj/sourceFile.txt",
                "type": "symbol",
                "recursive": true,
                "force": false
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/targetFile.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .link(
                LinkType::Symbol,
                "/u/jiahj/sourceFile.txt",
                "/u/jiahj/targetFile.txt",
            )
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
