use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;

use super::ObtainEnq;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DatasetCopy {}

impl TryFromResponse for DatasetCopy {
    async fn try_from_response(_value: reqwest::Response) -> Result<Self, crate::error::Error> {
        Ok(DatasetCopy {})
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{to_dataset}{to_member}")]
pub struct DatasetCopyBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(builder_fn = build_body)]
    from_dataset: Box<str>,
    #[endpoint(optional, skip_builder)]
    from_member: Option<Box<str>>,
    #[endpoint(optional, path)]
    volume: Box<str>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(optional, path)]
    to_member: Box<str>,
    #[endpoint(optional, skip_builder)]
    alias: Option<bool>,
    #[endpoint(optional, skip_builder)]
    enqueue: Option<ObtainEnq>,
    #[endpoint(optional, skip_builder)]
    replace: Option<bool>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    request: &'a str,
    from_dataset: FromDataset<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enq: Option<ObtainEnq>,
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
