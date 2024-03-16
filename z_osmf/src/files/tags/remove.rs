use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::Links;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct RemoveBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    links: Option<Links>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RemoveRequestJson {
    request: &'static str,
    action: &'static str,
    links: Option<Links>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &RemoveBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RemoveRequestJson {
        request: "chtag",
        action: "remove",
        links: builder.links,
        recursive: builder.recursive,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};

    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn remove_max() {
        let zosmf = get_zosmf();

        let json: Value = from_str(
            r#"
            {
                "request": "chtag",
                "action": "remove",
                "links": "suppress",
                "recursive": true
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
            .remove_tag("/u/jiahj/testFile.txt")
            .links(Links::Suppress)
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
