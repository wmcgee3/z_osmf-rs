use std::sync::Arc;

use zosmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct DatasetDelete {
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restifles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetDeleteBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: String,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: String,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: String,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<String>,
}

impl DatasetDeleteBuilder {
    pub async fn build(self) -> anyhow::Result<DatasetDelete> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        Ok(DatasetDelete { transaction_id })
    }
}

fn set_volume(mut builder: DatasetDeleteBuilder, value: String) -> DatasetDeleteBuilder {
    builder.volume = format!("-({})/", value);

    builder
}

fn set_member(mut builder: DatasetDeleteBuilder, value: String) -> DatasetDeleteBuilder {
    builder.member = format!("({})", value);

    builder
}
