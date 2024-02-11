use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileCopy {
    transaction_id: Box<str>,
}

impl TryFromResponse for FileCopy {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileCopy { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{to_path}")]
pub struct FileCopyBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_path: Box<str>,
    #[endpoint(path)]
    to_path: Box<str>,
    #[endpoint(optional, skip_builder)]
    overwrite: bool,
    #[endpoint(optional, skip_builder)]
    recursive: bool,
    #[endpoint(optional, skip_builder)]
    links: Option<FileCopyLinks>,
    #[endpoint(optional, skip_builder)]
    preserve: Option<FileCopyPreserve>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum FileCopyLinks {
    All,
    #[default]
    None,
    #[serde(rename = "src")]
    Source,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum FileCopyPreserve {
    All,
    #[serde(rename = "modtime")]
    ModificationTime,
    #[default]
    None,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    from: &'a str,
    overwrite: bool,
    recursive: bool,
    links: Option<FileCopyLinks>,
    preserve: Option<FileCopyPreserve>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCopyBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "copy",
        from: &builder.from_path,
        overwrite: builder.overwrite,
        recursive: builder.recursive,
        links: builder.links,
        preserve: builder.preserve,
    })
}
