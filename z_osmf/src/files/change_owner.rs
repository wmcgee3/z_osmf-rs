use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileChangeOwnerBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(builder_fn = build_body)]
    owner: Box<str>,
    #[endpoint(optional, skip_builder)]
    group: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    links: Option<ChangeOwnerLinks>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeOwnerLinks {
    Change,
    #[default]
    Follow,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    owner: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<ChangeOwnerLinks>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileChangeOwnerBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chown",
        owner: &builder.owner,
        group: builder.group.as_deref(),
        links: builder.links,
        recursive: builder.recursive,
    })
}
