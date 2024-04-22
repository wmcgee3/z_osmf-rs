use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::{get_member, get_volume};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds{volume}/{to_dataset}{to_member}")]
pub struct DatasetCopyBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_dataset: Box<str>,
    #[endpoint(skip_builder)]
    from_member: Option<Box<str>>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Box<str>>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(path, builder_fn = build_to_member)]
    to_member: Option<Box<str>>,
    #[endpoint(skip_builder)]
    alias: Option<bool>,
    #[endpoint(skip_builder)]
    enqueue: Option<DatasetCopyEnqueue>,
    #[endpoint(skip_builder)]
    replace: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DatasetCopyEnqueue {
    Exclu,
    Shr,
    Shrw,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    request: &'a str,
    from_dataset: FromDataset<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enq: Option<DatasetCopyEnqueue>,
    replace: Option<bool>,
}

#[derive(Serialize)]
struct FromDataset<'a> {
    dsn: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<bool>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetCopyBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "copy",
        from_dataset: FromDataset {
            dsn: &builder.from_dataset,
            member: builder.from_member.as_deref(),
            alias: builder.alias,
        },
        enq: builder.enqueue,
        replace: builder.replace,
    })
}

fn build_to_member<T>(builder: &DatasetCopyBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.to_member)
}

fn build_volume<T>(builder: &DatasetCopyBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_volume(&builder.volume)
}
