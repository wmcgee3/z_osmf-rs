use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_member, Enqueue};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{to_dataset}{to_member}")]
pub struct RenameBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_dataset: Box<str>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(skip_builder)]
    from_member: Option<Box<str>>,
    #[endpoint(path, builder_fn = build_to_member)]
    to_member: Option<Box<str>>,
    #[endpoint(skip_builder)]
    enqueue: Option<Enqueue>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    request: &'static str,
    from_dataset: FromDataset<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enq: &'a Option<Enqueue>,
}

#[derive(Serialize)]
struct FromDataset<'a> {
    dsn: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: &'a Option<Box<str>>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &RenameBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "rename",
        from_dataset: FromDataset {
            dsn: &builder.from_dataset,
            member: &builder.from_member,
        },
        enq: &builder.enqueue,
    })
}

fn build_to_member<T>(builder: &RenameBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.to_member)
}
