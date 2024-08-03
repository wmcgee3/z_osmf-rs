use std::marker::PhantomData;
use std::sync::Arc;

use chrono::NaiveDate;
use reqwest::RequestBuilder;
use serde::{Deserialize, Deserializer, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::restfiles::get_transaction_id;
use crate::{ClientCore, Result};

use super::{de_optional_y_n, ser_optional_y_n};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DatasetAttributesBase {
    #[serde(rename = "dsname")]
    name: Box<str>,
    #[serde(rename = "blksz")]
    block_size: Option<Box<str>>,
    #[serde(rename = "catnm")]
    catalog: Option<Box<str>>,
    #[getter(copy)]
    #[serde(default, deserialize_with = "de_optional_date", rename = "cdate")]
    creation_date: Option<NaiveDate>,
    #[serde(rename = "dev")]
    device_type: Option<Box<str>>,
    #[serde(rename = "dsntp")]
    dataset_type: Option<Box<str>>,
    #[serde(rename = "dsorg")]
    organization: Option<Box<str>>,
    #[getter(copy)]
    #[serde(default, deserialize_with = "de_optional_date", rename = "edate")]
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
    #[getter(copy)]
    #[serde(default, deserialize_with = "de_optional_date", rename = "rdate")]
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
    volume: DatasetVolume,
    #[serde(rename = "vols")]
    volumes: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DatasetAttributesName {
    #[serde(rename = "dsname")]
    name: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DatasetAttributesVolume {
    #[serde(rename = "dsname")]
    name: Box<str>,
    #[serde(rename = "vol")]
    volume: DatasetVolume,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
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

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds")]
pub struct DatasetListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(query = "dslevel")]
    level: Box<str>,
    #[endpoint(query = "volser")]
    volume: Option<Box<str>>,
    #[endpoint(query = "start")]
    start: Option<Box<str>>,
    #[endpoint(header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(skip_setter, builder_fn = build_attributes)]
    attributes: Option<Attrs>,
    #[endpoint(skip_builder)]
    include_total: Option<bool>,

    target_type: PhantomData<T>,
}

impl<T> DatasetListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn attributes_base(self) -> DatasetListBuilder<DatasetList<DatasetAttributesBase>> {
        DatasetListBuilder {
            core: self.core,
            level: self.level,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }

    pub fn attributes_dsname(self) -> DatasetListBuilder<DatasetList<DatasetAttributesName>> {
        DatasetListBuilder {
            core: self.core,
            level: self.level,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Dsname),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }

    pub fn attributes_vol(self) -> DatasetListBuilder<DatasetList<DatasetAttributesVolume>> {
        DatasetListBuilder {
            core: self.core,
            level: self.level,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            attributes: Some(Attrs::Vol),
            include_total: self.include_total,
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DatasetVolume {
    Alias,
    Migrated,
    Vsam,
    Volume(String),
}

impl<'de> Deserialize<'de> for DatasetVolume {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(match s.as_str() {
            "*ALIAS" => DatasetVolume::Alias,
            "MIGRAT" => DatasetVolume::Migrated,
            "*VSAM*" => DatasetVolume::Vsam,
            _ => DatasetVolume::Volume(s),
        })
    }
}

impl Serialize for DatasetVolume {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            DatasetVolume::Alias => "*ALIAS",
            DatasetVolume::Migrated => "MIGRAT",
            DatasetVolume::Volume(vol) => vol.as_ref(),
            DatasetVolume::Vsam => "*VSAM*",
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
        (None, Some(true)) => request_builder.header("X-IBM-Attributes", "dsname,total"),
        (Some(attributes), include_total) => request_builder.header(
            "X-IBM-Attributes",
            format!(
                "{}{}",
                attributes,
                if include_total == Some(true) {
                    ",total"
                } else {
                    ""
                }
            ),
        ),
        _ => request_builder,
    }
}

pub fn de_optional_date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<NaiveDate>, D::Error> {
    let s: String = Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "***None***" => Ok(None),
        s => Ok(Some(
            NaiveDate::parse_from_str(s, "%Y/%m/%d").map_err(serde::de::Error::custom)?,
        )),
    }
}

fn de_optional_yes_no<'de, D>(deserializer: D) -> std::result::Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| match s.as_str() {
            "YES" => Ok(true),
            "NO" => Ok(false),
            _ => Err(serde::de::Error::unknown_variant(&s, &["YES", "NO"])),
        })
        .transpose()
}

fn de_yes_no<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "YES" => Ok(true),
        "NO" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["YES", "NO"])),
    }
}

fn ser_optional_yes_no<S>(v: &Option<bool>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match v {
        Some(value) => serializer.serialize_str(if *value { "YES" } else { "NO" }),
        None => serializer.serialize_none(),
    }
}

fn ser_yes_no<S>(v: &bool, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *v { "YES" } else { "NO" })
}

#[cfg(test)]
mod tests {
    use serde::de::value::StrDeserializer;
    use serde::de::IntoDeserializer;

    use crate::tests::*;

    use super::*;

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

    #[test]
    fn test_de_optional_yes_no() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(default, deserialize_with = "de_optional_yes_no")]
            value: Option<bool>,
        }

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "YES"}"#).unwrap(),
            Test { value: Some(true) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": "NO"}"#).unwrap(),
            Test { value: Some(false) }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{"value": null}"#).unwrap(),
            Test { value: None }
        );

        assert_eq!(
            serde_json::from_str::<Test>(r#"{}"#).unwrap(),
            Test { value: None }
        );

        assert!(serde_json::from_str::<Test>(r#"{"value": "N"}"#).is_err());
    }

    #[test]
    fn test_de_yes_no() {
        let deserializer: StrDeserializer<serde::de::value::Error> = "YES".into_deserializer();
        assert!(de_yes_no(deserializer).unwrap());

        let deserializer: StrDeserializer<serde::de::value::Error> = "NO".into_deserializer();
        assert!(!de_yes_no(deserializer).unwrap());

        let deserializer: StrDeserializer<serde::de::value::Error> = "NONSENSE".into_deserializer();
        assert!(de_yes_no(deserializer).is_err());
    }

    #[test]
    fn test_ser_yes_no() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_yes_no(&true, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""YES""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_yes_no(&false, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""NO""#);
    }

    #[test]
    fn test_ser_optional_yes_no() {
        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&Some(true), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""YES""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&Some(false), &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#""NO""#);

        let mut serializer = serde_json::Serializer::new(Vec::new());
        ser_optional_yes_no(&None, &mut serializer).unwrap();
        let serialized = String::from_utf8(serializer.into_inner()).unwrap();
        assert_eq!(serialized, r#"null"#);
    }
}
