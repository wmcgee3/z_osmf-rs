use std::marker::PhantomData;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use zosmf_macros::{Endpoint, Getters};

use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetList<T> {
    items: Vec<T>,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
    transaction_id: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetBase {
    #[serde(rename = "dsname")]
    name: String,
    #[serde(rename = "blksz")]
    block_size: Option<String>,
    #[serde(rename = "catnm")]
    catalog: Option<String>,
    #[serde(rename = "cdate")]
    creation_date: Option<String>,
    #[serde(rename = "dev")]
    device_type: Option<String>,
    #[serde(rename = "dsntp")]
    dataset_type: Option<String>,
    #[serde(rename = "dsorg")]
    organization: Option<String>,
    #[serde(rename = "edate")]
    expiration_date: Option<String>,
    #[serde(rename = "extx")]
    extents_used: Option<String>,
    #[serde(rename = "lrecl")]
    logical_record_length: Option<String>,
    #[serde(
        rename = "migr",
        deserialize_with = "de_yes_no",
        serialize_with = "ser_yes_no"
    )]
    migrated: bool,
    #[serde(
        default,
        rename = "mvol",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    multi_volume: Option<bool>,
    #[serde(
        default,
        rename = "ovf",
        deserialize_with = "de_optional_yes_no",
        serialize_with = "ser_optional_yes_no"
    )]
    space_overflow: Option<bool>,
    #[serde(rename = "rdate")]
    last_referenced_date: Option<String>,
    #[serde(rename = "recfm")]
    record_format: Option<String>,
    #[serde(rename = "sizex")]
    size_in_tracks: Option<String>,
    #[serde(rename = "spacu")]
    space_units: Option<String>,
    #[serde(rename = "used")]
    percent_used: Option<String>,
    #[serde(rename = "vol")]
    volume: Volume,
    #[serde(rename = "vols")]
    volumes: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetName {
    #[serde(rename = "dsname")]
    name: String,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetVol {
    #[serde(rename = "dsname")]
    name: String,
    #[serde(rename = "vol")]
    volume: Volume,
}

#[derive(Clone, Debug)]
pub enum Volume {
    Alias,
    Migrated,
    Volume(String),
    Vsam,
}

impl<'de> Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "*ALIAS" => Volume::Alias,
            "MIGRAT" => Volume::Migrated,
            "*VSAM*" => Volume::Vsam,
            _ => Volume::Volume(s),
        })
    }
}

impl Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Volume::Alias => "*ALIAS",
            Volume::Migrated => "MIGRAT",
            Volume::Volume(vol) => vol,
            Volume::Vsam => "*VSAM*",
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds")]
pub struct DatasetListBuilder<'a, T>
where
    T: for<'de> Deserialize<'de>,
{
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(query = "dslevel")]
    name_pattern: String,
    #[endpoint(optional, query = "volser")]
    volume: Option<String>,
    #[endpoint(optional, query = "start")]
    start: Option<String>,
    #[endpoint(optional, header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_attributes")]
    attributes: Option<Attrs>,
    #[endpoint(optional, skip_builder)]
    include_total: bool,
    #[endpoint(optional, skip_setter, skip_builder)]
    attributes_marker: PhantomData<T>,
}

impl<'a, T> DatasetListBuilder<'a, T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> DatasetListBuilder<'a, DatasetBase> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            include_total: self.include_total,
            attributes_marker: PhantomData,
        }
    }

    pub fn attributes_dsname(self) -> DatasetListBuilder<'a, DatasetName> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Dsname),
            include_total: self.include_total,
            attributes_marker: PhantomData,
        }
    }

    pub fn attributes_vol(self) -> DatasetListBuilder<'a, DatasetVol> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Vol),
            include_total: self.include_total,
            attributes_marker: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<DatasetList<T>> {
        let response = self.get_response().await?;

        let transaction_id = get_transaction_id(&response)?;

        let ResponseJson {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
        } = response.json().await?;

        Ok(DatasetList {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
            transaction_id,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Attrs {
    Base,
    Dsname,
    Vol,
}

impl std::fmt::Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Attrs::Base => "base",
                Attrs::Dsname => "dsname",
                Attrs::Vol => "vol",
            }
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson<T> {
    items: Vec<T>,
    returned_rows: i32,
    #[serde(default)]
    more_rows: Option<bool>,
    #[serde(default)]
    total_rows: Option<i32>,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}

fn build_attributes<T>(
    request_builder: RequestBuilder,
    list_builder: &DatasetListBuilder<T>,
) -> RequestBuilder
where
    T: for<'de> Deserialize<'de>,
{
    match (list_builder.attributes, list_builder.include_total) {
        (None, false) => request_builder,
        (None, true) => request_builder.header("X-IBM-Attributes", "dsname,total"),
        (Some(attributes), include_total) => request_builder.header(
            "X-IBM-Attributes",
            format!(
                "{}{}",
                attributes,
                if include_total { ",total" } else { "" }
            ),
        ),
    }
}
