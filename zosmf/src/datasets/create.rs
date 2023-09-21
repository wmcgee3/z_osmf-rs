use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use zosmf_macros::{Endpoint, Getters};

use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Getters)]
pub struct DatasetCreate {
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/restfiles/ds/{dataset_name}")]
pub struct DatasetCreateBuilder {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = "build_json")]
    json: PhantomData<RequestJson<'static>>,

    #[endpoint(optional, skip_builder)]
    volume: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    device_type: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    organization: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    space_allocation_unit: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    primary_space: Option<i32>,
    #[endpoint(optional, skip_builder)]
    secondary_space: Option<i32>,
    #[endpoint(optional, skip_builder)]
    directory_blocks: Option<i32>,
    #[endpoint(optional, skip_builder)]
    average_block_size: Option<i32>,
    #[endpoint(optional, skip_builder)]
    record_format: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    block_size: Option<i32>,
    #[endpoint(optional, skip_builder)]
    logical_record_length: Option<i32>,
    #[endpoint(optional, skip_builder)]
    storage_class: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    management_class: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    data_class: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    dataset_type: Option<Box<str>>,
    #[endpoint(optional, skip_builder)]
    model_dataset: Option<Box<str>>,
}

impl DatasetCreateBuilder {
    pub async fn build(self) -> anyhow::Result<DatasetCreate> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        Ok(DatasetCreate { transaction_id })
    }
}

#[derive(Clone, Debug, Default, Serialize)]
struct RequestJson<'a> {
    #[serde(rename = "volser", skip_serializing_if = "Option::is_none")]
    volume: Option<&'a str>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    device_type: Option<&'a str>,
    #[serde(rename = "dsorg", skip_serializing_if = "Option::is_none")]
    organization: Option<&'a str>,
    #[serde(rename = "alcunit", skip_serializing_if = "Option::is_none")]
    space_allocation_unit: Option<&'a str>,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    primary_space: Option<&'a i32>,
    #[serde(rename = "secondary", skip_serializing_if = "Option::is_none")]
    secondary_space: Option<&'a i32>,
    #[serde(rename = "dirblk", skip_serializing_if = "Option::is_none")]
    directory_blocks: Option<&'a i32>,
    #[serde(rename = "avgblk", skip_serializing_if = "Option::is_none")]
    average_block_size: Option<&'a i32>,
    #[serde(rename = "recfm", skip_serializing_if = "Option::is_none")]
    record_format: Option<&'a str>,
    #[serde(rename = "blksize", skip_serializing_if = "Option::is_none")]
    block_size: Option<&'a i32>,
    #[serde(rename = "lrecl", skip_serializing_if = "Option::is_none")]
    logical_record_length: Option<&'a i32>,
    #[serde(rename = "storclass", skip_serializing_if = "Option::is_none")]
    storage_class: Option<&'a str>,
    #[serde(rename = "mgntclass", skip_serializing_if = "Option::is_none")]
    management_class: Option<&'a str>,
    #[serde(rename = "dataclass", skip_serializing_if = "Option::is_none")]
    data_class: Option<&'a str>,
    #[serde(rename = "dsntype", skip_serializing_if = "Option::is_none")]
    dataset_type: Option<&'a str>,
    #[serde(rename = "like", skip_serializing_if = "Option::is_none")]
    model_dataset: Option<&'a str>,
}

fn build_json(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetCreateBuilder,
) -> reqwest::RequestBuilder {
    let DatasetCreateBuilder {
        volume,
        device_type,
        organization,
        space_allocation_unit,
        primary_space,
        secondary_space,
        directory_blocks,
        average_block_size,
        record_format,
        block_size,
        logical_record_length,
        storage_class,
        management_class,
        data_class,
        dataset_type,
        model_dataset,
        ..
    } = builder;

    let request_json = RequestJson {
        volume: volume.as_deref(),
        device_type: device_type.as_deref(),
        organization: organization.as_deref(),
        space_allocation_unit: space_allocation_unit.as_deref(),
        primary_space: primary_space.as_ref(),
        secondary_space: secondary_space.as_ref(),
        directory_blocks: directory_blocks.as_ref(),
        average_block_size: average_block_size.as_ref(),
        record_format: record_format.as_deref(),
        block_size: block_size.as_ref(),
        logical_record_length: logical_record_length.as_ref(),
        storage_class: storage_class.as_deref(),
        management_class: management_class.as_deref(),
        data_class: data_class.as_deref(),
        dataset_type: dataset_type.as_deref(),
        model_dataset: model_dataset.as_deref(),
    };

    request_builder.json(&request_json)
}
