use std::marker::PhantomData;
use std::sync::Arc;

use chrono::NaiveDate;
use reqwest::RequestBuilder;
use serde::{Deserialize, Deserializer, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::{
    de_optional_y_n, de_optional_yes_no, de_yes_no, get_transaction_id, ser_optional_y_n,
    ser_optional_yes_no, ser_yes_no,
};
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetList<T> {
    items: Box<[T]>,
    #[getter(copy)]
    json_version: i32,
    #[getter(copy)]
    more_rows: Option<bool>,
    #[getter(copy)]
    returned_rows: i32,
    #[getter(copy)]
    total_rows: Option<i32>,
    transaction_id: Box<str>,
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

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetBase {
    #[serde(rename = "dsname")]
    name: Box<str>,
    #[serde(rename = "blksz")]
    block_size: Option<Box<str>>,
    #[serde(rename = "catnm")]
    catalog: Option<Box<str>>,
    #[serde(default, deserialize_with = "deserialize_optional_date", rename = "cdate")]
    creation_date: Option<NaiveDate>,
    #[serde(rename = "dev")]
    device_type: Option<Box<str>>,
    #[serde(rename = "dsntp")]
    dataset_type: Option<Box<str>>,
    #[serde(rename = "dsorg")]
    organization: Option<Box<str>>,
    #[serde(default, deserialize_with = "deserialize_optional_date", rename = "edate")]
    expiration_date: Option<NaiveDate>,
    #[serde(rename = "extx")]
    extents_used: Option<Box<str>>,
    #[serde(rename = "lrecl")]
    record_length: Option<Box<str>>,
    #[getter(copy)]
    #[serde(
        rename = "migr",
        deserialize_with = "de_yes_no",
        serialize_with = "ser_yes_no"
    )]
    migrated: bool,
    #[getter(copy)]
    #[serde(
        default,
        rename = "mvol",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    multi_volume: Option<bool>,
    #[getter(copy)]
    #[serde(
        default,
        rename = "ovf",
        deserialize_with = "de_optional_yes_no",
        serialize_with = "ser_optional_yes_no"
    )]
    space_overflow: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_date", rename = "rdate")]
    last_referenced_date: Option<NaiveDate>,
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
pub struct DatasetVolume {
    #[serde(rename = "dsname")]
    name: Box<str>,
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
    core: Arc<ClientCore>,

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
            core: self.core,
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
            core: self.core,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Dsname),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }

    pub fn attributes_vol(self) -> DatasetListBuilder<DatasetList<DatasetVolume>> {
        DatasetListBuilder {
            core: self.core,
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

pub fn deserialize_optional_date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error> {
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "***None***" => Ok(None),
        s => Ok(Some(
            NaiveDate::parse_from_str(s, "%Y/%m/%d").map_err(serde::de::Error::custom)?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/ds")
            .query(&[("dslevel", "IBMUSER.CONFIG.*")])
            .build()
            .unwrap();

        let list_datasets = zosmf
            .datasets()
            .list("IBMUSER.CONFIG.*")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", list_datasets)
        );
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/ds")
            .query(&[("dslevel", "**"), ("volser", "PEVTS2")])
            .header("X-IBM-Attributes", "base")
            .build()
            .unwrap();

        let list_datasets_base = zosmf
            .datasets()
            .list("**")
            .volume("PEVTS2")
            .attributes_base()
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", list_datasets_base)
        );
    }
}
