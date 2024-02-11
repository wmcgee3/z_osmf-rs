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
