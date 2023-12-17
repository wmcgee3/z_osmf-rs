use std::sync::Arc;

use z_osmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct FileDelete {
    transaction_id: Box<str>,
}

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/fs{file_path}")]
pub struct FileDeleteBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    file_path: Box<str>,
    #[endpoint(optional, builder_fn = "build_recursive")]
    recursive: bool,
}

impl FileDeleteBuilder {
    pub async fn build(self) -> anyhow::Result<FileDelete> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        Ok(FileDelete { transaction_id })
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
