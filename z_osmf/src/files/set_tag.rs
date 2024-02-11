use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::get_transaction_id;
use crate::ClientCore;

use super::{FileTagLinks, FileTagType};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileSetTag {
    transaction_id: Box<str>,
}

impl TryFromResponse for FileSetTag {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileSetTag { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileSetTagBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_body)]
    tag_type: Option<FileTagType>,
    #[endpoint(optional, skip_builder)]
    code_set: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    links: Option<FileTagLinks>,
    #[endpoint(optional, skip_builder)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Debug, Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    action: &'static str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    tag_type: Option<FileTagType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codeset: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<FileTagLinks>,
    recursive: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileSetTagBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "chtag",
        action: "set",
        tag_type: builder.tag_type,
        codeset: builder.code_set.as_deref(),
        links: builder.links,
        recursive: builder.recursive,
    })
}
