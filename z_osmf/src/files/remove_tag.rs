use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::FileTagLinks;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileRemoveTagBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    links: Option<FileTagLinks>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    action: &'static str,
    links: Option<FileTagLinks>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileRemoveTagBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chtag",
        action: "remove",
        links: builder.links,
        recursive: builder.recursive,
    })
}
