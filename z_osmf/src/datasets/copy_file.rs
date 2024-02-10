use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CopyFileToDataset {
    transaction_id: Box<str>,
}

impl TryFromResponse for CopyFileToDataset {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(CopyFileToDataset { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{to_dataset}{to_member}")]
pub struct CopyFileToDatasetBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    from_path: Box<str>,
    #[endpoint(optional, skip_builder)]
    file_type: Option<CopyFileType>,
    #[endpoint(optional, path, setter_fn = set_volume)]
    volume: Box<str>,
    #[endpoint(path)]
    to_dataset: Box<str>,
    #[endpoint(optional, path, setter_fn = set_to_member)]
    to_member: Box<str>,
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
        replace: builder.replace,
    })
}

fn set_to_member<T>(
    mut builder: CopyFileToDatasetBuilder<T>,
    value: Box<str>,
) -> CopyFileToDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.to_member = format!("({})", value).into();

    builder
}

fn set_volume<T>(
    mut builder: CopyFileToDatasetBuilder<T>,
    value: Box<str>,
) -> CopyFileToDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.volume = format!("-({})/", value).into();

    builder
}
