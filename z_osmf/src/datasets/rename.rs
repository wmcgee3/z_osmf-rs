use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::Enqueue;

pub struct DatasetRename {}

impl TryFromResponse for DatasetRename {
    async fn try_from_response(_value: reqwest::Response) -> Result<Self, crate::error::Error> {
        Ok(DatasetRename {})
    }
}

#[derive(Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{to_dataset}{to_member}")]
pub struct DatasetRenameBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(skip_setter, builder_fn = build_body)]
    from_dataset: Box<str>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(optional, skip_builder)]
    from_member: Option<Box<str>>,
    #[endpoint(optional, path, setter_fn = set_to_member)]
    to_member: Box<str>,
    #[endpoint(optional, skip_builder)]
    enqueue: Option<Enqueue>,

    #[endpoint(optional, skip_setter, skip_builder)]
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
    builder: &DatasetRenameBuilder<T>,
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

fn set_to_member<T>(
    mut builder: DatasetRenameBuilder<T>,
    value: Box<str>,
) -> DatasetRenameBuilder<T>
where
    T: TryFromResponse,
{
    builder.to_member = format!("({})", value).into();

    builder
}
