use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DeleteFile {
    transaction_id: Box<str>,
}

impl TryFromResponse for DeleteFile {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(DeleteFile { transaction_id })
    }
}

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{path}")]
pub struct DeleteFileBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_recursive)]
    recursive: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn build_recursive<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &DeleteFileBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.recursive {
        request_builder = request_builder.header("X-IBM-Option", "recursive");
    }

    request_builder
}
