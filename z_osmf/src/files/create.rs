use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::RequestBuilder;
use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::restfiles::get_transaction_id;

#[derive(Clone, Debug)]
pub struct FileCreate {
    pub transaction_id: Box<str>,
}

impl TryFromResponse for FileCreate {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileCreate { transaction_id })
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
    Directory,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/restfiles/fs{path}")]
pub struct FileCreateBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = build_json)]
    json: PhantomData<RequestJson<'static>>,

    #[endpoint(optional, skip_builder)]
    file_type: Option<FileType>,
    #[endpoint(optional, skip_builder)]
    mode: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    file_type: Option<&'a FileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<&'a str>,
}

fn build_json<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCreateBuilder<T>,
) -> RequestBuilder
where
    T: TryFromResponse,
{
    let FileCreateBuilder {
        file_type, mode, ..
    } = builder;

    request_builder.json(&RequestJson {
        file_type: file_type.as_ref(),
        mode: mode.as_deref(),
    })
}
