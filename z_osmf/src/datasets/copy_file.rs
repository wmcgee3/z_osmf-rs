use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::restfiles::CopyDataType;
use crate::ClientCore;

use super::{get_member, get_volume};

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds{volume}/{to_dataset}{to_member}")]
pub struct DatasetCopyFileBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_path: Arc<str>,
    #[endpoint(skip_builder)]
    file_type: Option<CopyDataType>,
    #[endpoint(path, builder_fn = build_volume)]
    volume: Option<Arc<str>>,
    #[endpoint(path)]
    to_dataset: Arc<str>,
    #[endpoint(path, builder_fn = build_to_member)]
    to_member: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    replace: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    request: &'a str,
    from_file: FromFile<'a>,
    replace: Option<bool>,
}

#[derive(Serialize)]
struct FromFile<'a> {
    filename: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<CopyDataType>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetCopyFileBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "copy",
        from_file: FromFile {
            filename: &builder.from_path,
            file_type: builder.file_type,
        },
        replace: builder.replace,
    })
}

fn build_to_member<T>(builder: &DatasetCopyFileBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_member(&builder.to_member)
}

fn build_volume<T>(builder: &DatasetCopyFileBuilder<T>) -> String
where
    T: TryFromResponse,
{
    get_volume(&builder.volume)
}
