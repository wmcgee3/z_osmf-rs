use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct ChangeOwnerBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(builder_fn = build_body)]
    owner: Box<str>,
    #[endpoint(skip_builder)]
    group: Option<Box<str>>,
    #[endpoint(skip_builder)]
    links: Option<Links>,
    #[endpoint(skip_builder)]
    recursive: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Links {
    Change,
    Follow,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    owner: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &ChangeOwnerBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chown",
        owner: &builder.owner,
        group: builder.group.as_deref(),
        links: builder.links,
        recursive: builder.recursive == Some(true),
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::{get_zosmf, GetJson};

    use super::*;

    #[test]
    fn maximal_request() {
        let zosmf = get_zosmf();

        let json: serde_json::Value = serde_json::from_str(
            r#"
        {
            "request": "chown",
            "owner": "ibmuser",
            "group": "ibmgrp",
            "links": "change",
            "recursive": true
        }
        "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/testDir")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .change_owner("/u/jiahj/testDir", "ibmuser")
            .group("ibmgrp")
            .links(Links::Change)
            .recursive(true)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }

    #[test]
    fn minimal_request() {
        let zosmf = get_zosmf();

        let json: serde_json::Value = serde_json::from_str(
            r#"
        {
            "request": "chown",
            "owner": "ibmuser",
            "recursive": false
        }
        "#,
        )
        .unwrap();
        let manual_request = zosmf
            .core
            .client
            .put("https://test.com/zosmf/restfiles/fs/u/jiahj/test.txt")
            .json(&json)
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .change_owner("/u/jiahj/test.txt", "ibmuser")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
        assert_eq!(manual_request.json(), request.json());
    }
}
