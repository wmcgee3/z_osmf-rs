use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;

use super::ObtainEnq;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct CopyFileToDataset {}

impl TryFromResponse for CopyFileToDataset {
    async fn try_from_response(_value: reqwest::Response) -> Result<Self, crate::error::Error> {
        Ok(CopyFileToDataset {})
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{to_dataset}{to_member}")]
pub struct CopyFileToDatasetBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(builder_fn = build_body)]
    from_path: Box<str>,
    #[endpoint(optional, skip_builder)]
    file_type: Option<CopyFileType>,
    #[endpoint(optional, path)]
    volume: Box<str>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(optional, path)]
    to_member: Box<str>,
    #[endpoint(optional, skip_builder)]
    enqueue: Option<ObtainEnq>,
    #[endpoint(optional, skip_builder)]
    replace: Option<bool>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum CopyFileType {
    Binary,
    Executable,
    Text,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'a str,
    from_file: FromFile<'a>,
    enq: Option<ObtainEnq>,
    replace: Option<bool>,
}

#[derive(Serialize)]
struct FromFile<'a> {
    filename: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_type: Option<CopyFileType>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &CopyFileToDatasetBuilder<T>,
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
        enq: builder.enqueue,
        replace: builder.replace,
    })
}
