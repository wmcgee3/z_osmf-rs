use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{to_path}")]
pub struct FileCopyBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_path: Arc<str>,
    #[endpoint(path)]
    to_path: Arc<str>,
    #[endpoint(skip_builder)]
    overwrite: Option<bool>,
    #[endpoint(skip_builder)]
    recursive: Option<bool>,
    #[endpoint(skip_builder)]
    links: Option<FileCopyLinks>,
    #[endpoint(skip_builder)]
    preserve: Option<FileCopyPreserve>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileCopyLinks {
    All,
    None,
    Src,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileCopyPreserve {
    All,
    Modtime,
    None,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    from: &'a str,
    overwrite: bool,
    recursive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<FileCopyLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve: Option<FileCopyPreserve>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCopyBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "copy",
        from: &builder.from_path,
        overwrite: builder.overwrite == Some(true),
        recursive: builder.recursive == Some(true),
        links: builder.links,
        preserve: builder.preserve,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "copy",
                "from": "/u/jiahj/sourceDir",
                "overwrite": true,
                "recursive": true,
                "links": "src",
                "preserve": "modtime"
            }
            "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/targetDir")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .copy("/u/jiahj/sourceDir", "/u/jiahj/targetDir")
            .overwrite(true)
            .recursive(true)
            .links(FileCopyLinks::Src)
            .preserve(FileCopyPreserve::Modtime)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }

    #[test]
    fn minimal_request() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "copy",
                "from": "/u/jiahj/sourceFile.txt",
                "overwrite": false,
                "recursive": false
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
            .copy("/u/jiahj/sourceFile.txt", "/u/jiahj/targetFile.txt")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
