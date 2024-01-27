use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::RequestBuilder;
use serde::Serialize;
use z_osmf_core::error::Error;
use z_osmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct FileCreate {
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for FileCreate {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
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
pub struct FileCreateBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = "build_json")]
    json: PhantomData<RequestJson<'static>>,

    #[endpoint(optional, skip_builder)]
    file_type: Option<FileType>,
    #[endpoint(optional, skip_builder)]
    mode: Option<Box<str>>,
}

impl FileCreateBuilder {
    pub async fn build(self) -> Result<FileCreate, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

#[derive(Serialize)]
struct RequestJson<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    file_type: Option<&'a FileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<&'a str>,
}

fn build_json(
    request_builder: reqwest::RequestBuilder,
    builder: &FileCreateBuilder,
) -> RequestBuilder {
    let FileCreateBuilder {
        file_type, mode, ..
    } = builder;

    request_builder.json(&RequestJson {
        file_type: file_type.as_ref(),
        mode: mode.as_deref(),
    })
}
