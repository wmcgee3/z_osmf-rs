use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use z_osmf_macros::{Endpoint, Getters};

use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetList<T> {
    items: Vec<T>,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
    transaction_id: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetBase {
    #[serde(rename = "dsname")]
    name: Box<str>,
    #[serde(rename = "blksz")]
    block_size: Option<Box<str>>,
    #[serde(rename = "catnm")]
    catalog: Option<Box<str>>,
    #[serde(rename = "cdate")]
    creation_date: Option<Box<str>>,
    #[serde(rename = "dev")]
    device_type: Option<Box<str>>,
    #[serde(rename = "dsntp")]
    dataset_type: Option<Box<str>>,
    #[serde(rename = "dsorg")]
    organization: Option<Box<str>>,
    #[serde(rename = "edate")]
    expiration_date: Option<Box<str>>,
    #[serde(rename = "extx")]
    extents_used: Option<Box<str>>,
    #[serde(rename = "lrecl")]
    logical_record_length: Option<Box<str>>,
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
    last_referenced_date: Option<Box<str>>,
    #[serde(rename = "recfm")]
    record_format: Option<Box<str>>,
    #[serde(rename = "sizex")]
    size_in_tracks: Option<Box<str>>,
    #[serde(rename = "spacu")]
    space_units: Option<Box<str>>,
    #[serde(rename = "used")]
    percent_used: Option<Box<str>>,
    #[serde(rename = "vol")]
    volume: Volume,
    #[serde(rename = "vols")]
    volumes: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetName {
    #[serde(rename = "dsname")]
    name: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetVol {
    #[serde(rename = "dsname")]
    name: Box<str>,
    #[serde(rename = "vol")]
    volume: Volume,
}

#[derive(Clone, Debug)]
pub enum Volume {
    Alias,
    Migrated,
    Volume(Box<str>),
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
            _ => Volume::Volume(s.into()),
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
            Volume::Volume(vol) => vol.as_ref(),
            Volume::Vsam => "*VSAM*",
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/z_osmf/restfiles/ds")]
pub struct DatasetListBuilder<T>
where
    T: for<'de> Deserialize<'de>,
{
    base_url: Arc<str>,
    client: Client,

    #[endpoint(query = "dslevel")]
    name_pattern: Box<str>,
    #[endpoint(optional, query = "volser")]
    volume: Option<Box<str>>,
    #[endpoint(optional, query = "start")]
    start: Option<Box<str>>,
    #[endpoint(optional, header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_attributes")]
    attributes: Option<Attrs>,
    #[endpoint(optional, skip_builder)]
    include_total: bool,
    #[endpoint(optional, skip_setter, skip_builder)]
    attributes_marker: PhantomData<T>,
}

impl<T> DatasetListBuilder<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> DatasetListBuilder<DatasetBase> {
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

    pub fn attributes_dsname(self) -> DatasetListBuilder<DatasetName> {
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

    pub fn attributes_vol(self) -> DatasetListBuilder<DatasetVol> {
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
