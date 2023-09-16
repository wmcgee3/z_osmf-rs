use reqwest::{Client, RequestBuilder};
use zosmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct FileDelete {
    transaction_id: String,
}

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{file_path}")]
pub struct FileDeleteBuilder<'a> {
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    file_path: String,
    #[endpoint(optional, builder_fn = "build_recursive")]
    recursive: bool,
}

impl<'a> FileDeleteBuilder<'a> {
    pub async fn build(self) -> anyhow::Result<FileDelete> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        Ok(FileDelete { transaction_id })
    }
}

fn build_recursive(
    mut request_builder: RequestBuilder,
    builder: &FileDeleteBuilder,
) -> RequestBuilder {
    if builder.recursive {
        request_builder = request_builder.header("X-IBM-Option", "recursive");
    }

    request_builder
}
