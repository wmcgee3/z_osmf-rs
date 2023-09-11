use std::marker::PhantomData;

use anyhow::Context;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use zosmf_macros::{Endpoint, Getter};

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct DatasetList<A>
where
    A: Attr,
{
    items: Vec<A>,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
    transaction_id: String,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct DatasetBase {
    dsname: String,
    blksz: Option<String>,
    catnm: Option<String>,
    cdate: Option<String>,
    dev: Option<String>,
    dsntp: Option<String>,
    dsorg: Option<String>,
    edate: Option<String>,
    extx: Option<String>,
    lrecl: Option<String>,
    #[serde(deserialize_with = "de_migr", serialize_with = "ser_migr")]
    migr: bool,
    #[serde(deserialize_with = "de_mvol", serialize_with = "ser_mvol")]
    mvol: Option<bool>,
    ovf: Option<String>,
    rdate: Option<String>,
    recfm: Option<String>,
    sizex: Option<String>,
    spacu: Option<String>,
    used: Option<String>,
    vol: Volume,
    vols: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct DatasetName {
    dsname: String,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct DatasetVol {
    dsname: String,
    vol: Volume,
}

#[derive(Clone, Debug)]
pub enum Volume {
    Alias,
    Migrate,
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
            "MIGRAT" => Volume::Migrate,
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
            Volume::Migrate => "MIGRAT",
            Volume::Volume(vol) => vol,
            Volume::Vsam => "*VSAM*",
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds")]
pub struct DatasetListBuilder<'a, A>
where
    A: Attr + for<'de> Deserialize<'de>,
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
    attrs: PhantomData<A>,
}

impl<'a, A> DatasetListBuilder<'a, A>
where
    A: Attr + for<'de> Deserialize<'de>,
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
            attrs: PhantomData,
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
            attrs: PhantomData,
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
            attrs: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<DatasetList<A>> {
        let response = self.get_response().await?;

        let transaction_id = response
            .headers()
            .get("X-IBM-Txid")
            .context("missing transaction id")?
            .to_str()?
            .to_string();

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

pub trait Attr {}
impl Attr for DatasetBase {}
impl Attr for DatasetName {}
impl Attr for DatasetVol {}

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
struct ResponseJson<A>
where
    A: Attr,
{
    items: Vec<A>,
    returned_rows: i32,
    #[serde(default)]
    more_rows: Option<bool>,
    #[serde(default)]
    total_rows: Option<i32>,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}

fn build_attributes<A>(
    request_builder: RequestBuilder,
    list_builder: &DatasetListBuilder<A>,
) -> RequestBuilder
where
    A: Attr + for<'de> Deserialize<'de>,
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

fn de_migr<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(s == "YES")
}

fn ser_migr<S>(migr: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *migr { "YES" } else { "NO" })
}

fn de_mvol<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.map(|s| s == "Y"))
}

fn ser_mvol<S>(mvol: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(mvol) = mvol {
        serializer.serialize_str(if *mvol { "Y" } else { "N" })
    } else {
        serializer.serialize_none()
    }
}
