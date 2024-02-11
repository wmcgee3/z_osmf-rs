use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileRename {
    transaction_id: Box<str>,
}

impl TryFromResponse for FileRename {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileRename { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{to_path}")]
pub struct FileRenameBuilder<T>
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

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson<'a> {
    request: &'static str,
    from: &'a str,
    overwrite: bool,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileRenameBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "move",
        from: &builder.from_path,
        overwrite: builder.overwrite,
    })
}
