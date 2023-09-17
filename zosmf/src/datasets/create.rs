use std::sync::Arc;

use zosmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct DatasetCreate {
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/restfiles/ds/{dataset_name}")]
pub struct DatasetCreateBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: String,
}

impl DatasetCreateBuilder {
    pub async fn build(self) -> anyhow::Result<DatasetCreate> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        Ok(DatasetCreate { transaction_id })
    }
}
