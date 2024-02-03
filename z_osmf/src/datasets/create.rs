use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct CreateDataset {
    transaction_id: Box<str>,
}

impl TryFromResponse for CreateDataset {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(CreateDataset { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/restfiles/ds/{dataset_name}")]
pub struct CreateDatasetBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = build_json)]
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
    record_length: Option<i32>,
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

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
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
    record_length: Option<&'a i32>,
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

fn build_json<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &CreateDatasetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let CreateDatasetBuilder {
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
        record_length,
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
        record_length: record_length.as_ref(),
        storage_class: storage_class.as_deref(),
        management_class: management_class.as_deref(),
        data_class: data_class.as_deref(),
        dataset_type: dataset_type.as_deref(),
        model_dataset: model_dataset.as_deref(),
    };

    request_builder.json(&request_json)
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn test_example_1() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "volser":"zmf046",
            "unit":"3390",
            "dsorg":"PS",
            "alcunit":"TRK",
            "primary":10,
            "secondary":5,
            "avgblk":500,
            "recfm":"FB",
            "blksize":400,
            "lrecl":80
        }
        "#;

        let manual_request = zosmf
            .client
            .post("https://example.com/zosmf/restfiles/ds/test.dataset")
            .json(&serde_json::from_str::<serde_json::Value>(raw_json).unwrap())
            .build()
            .unwrap();

        let create_dataset = zosmf
            .datasets()
            .create("test.dataset")
            .volume("zmf046")
            .device_type("3390")
            .organization("PS")
            .space_allocation_unit("TRK")
            .primary_space(10)
            .secondary_space(5)
            .average_block_size(500)
            .record_format("FB")
            .block_size(400)
            .record_length(80)
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", create_dataset)
        );

        assert_eq!(
            manual_request.json().unwrap(),
            create_dataset.json().unwrap()
        );
    }

    #[test]
    fn test_example_2() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "volser": "zmf046",
            "unit": "3390",
            "dsorg": "PO",
            "alcunit": "TRK",
            "primary": 10,
            "secondary": 5,
            "dirblk": 10,
            "avgblk": 500,
            "recfm": "FB",
            "blksize": 400,
            "lrecl": 80
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();

        let manual_request = zosmf
            .client
            .post("https://example.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.NEWDS02")
            .json(&json)
            .build()
            .unwrap();

        let create_dataset = zosmf
            .datasets()
            .create("JIAHJ.REST.TEST.NEWDS02")
            .volume("zmf046")
            .device_type("3390")
            .organization("PO")
            .space_allocation_unit("TRK")
            .primary_space(10)
            .secondary_space(5)
            .directory_blocks(10)
            .average_block_size(500)
            .record_format("FB")
            .block_size(400)
            .record_length(80)
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", create_dataset)
        );

        assert_eq!(
            manual_request.json().unwrap(),
            create_dataset.json().unwrap()
        );
    }

    #[test]
    fn test_example_3() {
        let zosmf = get_zosmf();

        let raw_json = r#"
        {
            "volser": "zmf046",
            "unit": "3390",
            "dsorg": "PO",
            "alcunit": "TRK",
            "primary": 10,
            "secondary": 5,
            "dirblk": 10,
            "avgblk": 500,
            "recfm": "FB",
            "blksize": 400,
            "lrecl": 80,
            "dsntype": "LIBRARY"
        }
        "#;
        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();

        let manual_request = zosmf
            .client
            .post("https://example.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.NEWDS02")
            .json(&json)
            .build()
            .unwrap();

        let create_pdse = zosmf
            .datasets()
            .create("JIAHJ.REST.TEST.NEWDS02")
            .volume("zmf046")
            .device_type("3390")
            .organization("PO")
            .space_allocation_unit("TRK")
            .primary_space(10)
            .secondary_space(5)
            .directory_blocks(10)
            .average_block_size(500)
            .record_format("FB")
            .block_size(400)
            .record_length(80)
            .dataset_type("LIBRARY")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", create_pdse)
        );

        assert_eq!(manual_request.json().unwrap(), create_pdse.json().unwrap());
    }
}
