use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DeleteDataset {
    transaction_id: Box<str>,
}

impl TryFromResponse for DeleteDataset {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(DeleteDataset { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DeleteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = set_volume)]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn set_member<T>(mut builder: DeleteDatasetBuilder<T>, value: Box<str>) -> DeleteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<T>(mut builder: DeleteDatasetBuilder<T>, value: Box<str>) -> DeleteDatasetBuilder<T>
where
    T: TryFromResponse,
{
    builder.volume = format!("-({})/", value).into();

    builder
}
