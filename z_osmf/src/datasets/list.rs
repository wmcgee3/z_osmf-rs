use std::marker::PhantomData;
use std::sync::Arc;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::error::Error;
use crate::restfiles::get_transaction_id;
use crate::utils::{
    de_optional_y_n, de_optional_yes_no, de_yes_no, ser_optional_y_n, ser_optional_yes_no,
    ser_yes_no,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetList<T> {
    pub items: Box<[T]>,
    pub json_version: i32,
    pub more_rows: Option<bool>,
    pub returned_rows: i32,
    pub total_rows: Option<i32>,
    pub transaction_id: Box<str>,
}

impl<T> TryFromResponse for DatasetList<T>
where
    T: for<'de> Deserialize<'de>,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        let ResponseJson {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
        } = value.json().await?;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetBase {
    #[serde(rename = "dsname")]
    pub name: Box<str>,
    #[serde(rename = "blksz")]
    pub block_size: Option<Box<str>>,
    #[serde(rename = "catnm")]
    pub catalog: Option<Box<str>>,
    #[serde(rename = "cdate")]
    pub creation_date: Option<Box<str>>,
    #[serde(rename = "dev")]
    pub device_type: Option<Box<str>>,
    #[serde(rename = "dsntp")]
    pub dataset_type: Option<Box<str>>,
    #[serde(rename = "dsorg")]
    pub organization: Option<Box<str>>,
    #[serde(rename = "edate")]
    pub expiration_date: Option<Box<str>>,
    #[serde(rename = "extx")]
    pub extents_used: Option<Box<str>>,
    #[serde(rename = "lrecl")]
    pub logical_record_length: Option<Box<str>>,
    #[serde(
        rename = "migr",
        deserialize_with = "de_yes_no",
        serialize_with = "ser_yes_no"
    )]
    pub migrated: bool,
    #[serde(
        default,
        rename = "mvol",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    pub multi_volume: Option<bool>,
    #[serde(
        default,
        rename = "ovf",
        deserialize_with = "de_optional_yes_no",
        serialize_with = "ser_optional_yes_no"
    )]
    pub space_overflow: Option<bool>,
    #[serde(rename = "rdate")]
    pub last_referenced_date: Option<Box<str>>,
    #[serde(rename = "recfm")]
    pub record_format: Option<Box<str>>,
    #[serde(rename = "sizex")]
    pub size_in_tracks: Option<Box<str>>,
    #[serde(rename = "spacu")]
    pub space_units: Option<Box<str>>,
    #[serde(rename = "used")]
    pub percent_used: Option<Box<str>>,
    #[serde(rename = "vol")]
    pub volume: Volume,
    #[serde(rename = "vols")]
    pub volumes: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetName {
    #[serde(rename = "dsname")]
    pub name: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatasetVol {
    #[serde(rename = "dsname")]
    pub name: Box<str>,
    #[serde(rename = "vol")]
    pub volume: Volume,
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
#[endpoint(method = get, path = "/zosmf/restfiles/ds")]
pub struct DatasetListBuilder<T>
where
    T: TryFromResponse,
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
    #[endpoint(optional, skip_setter, builder_fn = build_attributes)]
    attributes: Option<Attrs>,
    #[endpoint(optional, skip_builder)]
    include_total: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> DatasetListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn attributes_base(self) -> DatasetListBuilder<DatasetList<DatasetBase>> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }

    pub fn attributes_dsname(self) -> DatasetListBuilder<DatasetList<DatasetName>> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Dsname),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }

    pub fn attributes_vol(self) -> DatasetListBuilder<DatasetList<DatasetVol>> {
        DatasetListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Vol),
            include_total: self.include_total,
            target_type: PhantomData,
        }
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
    items: Box<[T]>,
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
    T: TryFromResponse,
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
