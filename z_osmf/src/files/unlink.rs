use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct UnlinkBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(builder_fn = build_body)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    _builder: &UnlinkBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson { request: "unlink" })
}
