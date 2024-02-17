use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileChangeModeBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(builder_fn = build_body)]
    mode: Box<str>,
    #[endpoint(optional, skip_builder)]
    links: Option<FileModeLinks>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileModeLinks {
    #[default]
    Follow,
    Suppress,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    mode: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<FileModeLinks>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileChangeModeBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chmod",
        mode: &builder.mode,
        links: builder.links,
        recursive: builder.recursive,
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let json = r#"
        {
            "request": "chmod",
            "mode": "755",
            "links": "suppress",
            "recursive": true
        }
        "#;
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/text.txt")
            .json(&serde_json::from_str::<serde_json::Value>(json).unwrap())
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .change_mode("/u/jiahj/text.txt", "755")
            .links(FileModeLinks::Suppress)
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }

    #[test]
    fn minimal_request() {
        let zosmf = get_zosmf();

        let json = r#"
        {
            "request": "chmod",
            "mode": "755",
            "recursive": false
        }
        "#;
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/text.txt")
            .json(&serde_json::from_str::<serde_json::Value>(json).unwrap())
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .change_mode("/u/jiahj/text.txt", "755")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
