use std::sync::Arc;

use z_osmf_core::error::Error;
use z_osmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct FileDelete {
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for FileDelete {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(FileDelete { transaction_id })
    }
}

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{path}")]
pub struct FileDeleteBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = "build_recursive")]
    recursive: bool,
}

impl FileDeleteBuilder {
    pub async fn build(self) -> Result<FileDelete, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

fn build_recursive(
    mut request_builder: reqwest::RequestBuilder,
    builder: &FileDeleteBuilder,
) -> reqwest::RequestBuilder {
    if builder.recursive {
        request_builder = request_builder.header("X-IBM-Option", "recursive");
    }

    request_builder
}
