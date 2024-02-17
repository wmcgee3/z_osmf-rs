use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{name}{member}")]
pub struct DatasetRecallBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    name: Box<str>,
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    wait: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    wait: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetRecallBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "hrecall",
        wait: builder.wait,
    })
}

fn set_member<T>(mut builder: DatasetRecallBuilder<T>, value: Box<str>) -> DatasetRecallBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}
