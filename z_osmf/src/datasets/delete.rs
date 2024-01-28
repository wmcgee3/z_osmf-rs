use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::restfiles::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct DatasetDelete {
    transaction_id: Box<str>,
}

impl TryFromResponse for DatasetDelete {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(DatasetDelete { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetDeleteBuilder<T> {
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

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> DatasetDeleteBuilder<T> {
    pub async fn build(self) -> Result<DatasetDelete, Error> {
        let response = self.get_response().await?;

        response.try_into_target().await
    }
}

fn set_volume<T>(mut builder: DatasetDeleteBuilder<T>, value: Box<str>) -> DatasetDeleteBuilder<T> {
    builder.volume = format!("-({})/", value).into();

    builder
}

fn set_member<T>(mut builder: DatasetDeleteBuilder<T>, value: Box<str>) -> DatasetDeleteBuilder<T> {
    builder.member = format!("({})", value).into();

    builder
}
