use std::sync::Arc;

use z_osmf_macros::{Endpoint, Getters};

use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct DatasetDelete {
    transaction_id: Box<str>,
}

impl TryFrom<reqwest::Response> for DatasetDelete {
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(DatasetDelete { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetDeleteBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: Box<str>,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,
}

impl DatasetDeleteBuilder {
    pub async fn build(self) -> Result<DatasetDelete, Error> {
        let response = self.get_response().await?;

        response.try_into()
    }
}

fn set_volume(mut builder: DatasetDeleteBuilder, value: Box<str>) -> DatasetDeleteBuilder {
    builder.volume = format!("-({})/", value).into();

    builder
}

fn set_member(mut builder: DatasetDeleteBuilder, value: Box<str>) -> DatasetDeleteBuilder {
    builder.member = format!("({})", value).into();

    builder
}
