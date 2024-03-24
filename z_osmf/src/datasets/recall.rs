use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::get_member;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{name}{member}")]
pub struct RecallBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    name: Box<str>,
    #[endpoint(path, builder_fn = build_member)]
    member: Option<Box<str>>,
    #[endpoint(builder_fn = build_body)]
    wait: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    wait: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &RecallBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "hrecall",
        wait: builder.wait == Some(true),
    })
}

fn build_member<T>(builder: &RecallBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.member)
}
