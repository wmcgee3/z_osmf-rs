use std::marker::PhantomData;

use anyhow::Context;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use zosmf_macros::Endpoint;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct List<A>
where
    A: Attributes,
{
    pub items: Vec<A>,
    pub json_version: i32,
    pub more_rows: Option<bool>,
    pub returned_rows: i32,
    pub total_rows: Option<i32>,
    pub transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds")]
pub struct ListBuilder<'a, A>
where
    A: Attributes,
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
    #[endpoint(optional, skip_builder)]
    include_total: bool,
    #[endpoint(optional, skip_setter, skip_builder)]
    attributes: PhantomData<A>,
}

impl<'a, A> ListBuilder<'a, A>
where
    A: Attributes,
{
    pub fn attributes_base(self) -> ListBuilder<'a, Base> {
        ListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            include_total: self.include_total,
            attributes: PhantomData,
        }
    }

    pub fn attributes_dsname(self) -> ListBuilder<'a, Dsname> {
        ListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            include_total: self.include_total,
            attributes: PhantomData,
        }
    }

    pub fn attributes_vol(self) -> ListBuilder<'a, Vol> {
        ListBuilder {
            base_url: self.base_url,
            client: self.client,
            name_pattern: self.name_pattern,
            volume: self.volume,
            start: self.start,
            max_items: self.max_items,
            include_total: self.include_total,
            attributes: PhantomData,
        }
    }
}

impl<'a> ListBuilder<'a, Base> {
    pub async fn build(self) -> anyhow::Result<List<Base>> {
        let response = self
            .get_request_builder()
            .header(
                "X-IBM-Attributes",
                format!("base{}", if self.include_total { ",total" } else { "" }),
            )
            .send()
            .await?
            .error_for_status()?;

        Ok(get_list(response).await?)
    }
}

impl<'a> ListBuilder<'a, Dsname> {
    pub async fn build(self) -> anyhow::Result<List<Dsname>> {
        let response = self
            .get_request_builder()
            .header(
                "X-IBM-Attributes",
                if self.include_total {
                    "dsname,total"
                } else {
                    ""
                },
            )
            .send()
            .await?
            .error_for_status()?;

        Ok(get_list(response).await?)
    }
}

impl<'a> ListBuilder<'a, Vol> {
    pub async fn build(self) -> anyhow::Result<List<Vol>> {
        let response = self
            .get_request_builder()
            .header(
                "X-IBM-Attributes",
                format!("vol{}", if self.include_total { ",total" } else { "" }),
            )
            .send()
            .await?
            .error_for_status()?;

        Ok(get_list(response).await?)
    }
}

pub trait Attributes {}

#[derive(Deserialize)]
pub struct Base {
    pub dsname: String,
    pub blksz: Option<String>,
    pub catnm: Option<String>,
    pub cdate: Option<String>,
    pub dev: Option<String>,
    pub dsntp: Option<String>,
    pub dsorg: Option<String>,
    pub edate: Option<String>,
    pub extx: Option<String>,
    pub lrecl: Option<String>,
    pub migr: Option<String>,
    pub mvol: Option<String>,
    pub ovf: Option<String>,
    pub rdate: Option<String>,
    pub recfm: Option<String>,
    pub sizex: Option<String>,
    pub spacu: Option<String>,
    pub used: Option<String>,
    pub vol: String,
    pub vols: Option<String>,
}

impl Attributes for Base {}

#[derive(Deserialize)]
pub struct Dsname {
    pub dsname: String,
}

impl Attributes for Dsname {}

#[derive(Deserialize)]
pub struct Vol {
    pub dsname: String,
    pub vol: String,
}

impl Attributes for Vol {}

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

async fn get_list<A>(response: reqwest::Response) -> anyhow::Result<List<A>>
where
    A: Attributes + DeserializeOwned,
{
    let transaction_id = response
        .headers()
        .get("X-IBM-Txid")
        .context("missing transaction id")?
        .to_str()?
        .to_string();

    let ResponseJson {
        items,
        returned_rows,
        more_rows,
        total_rows,
        json_version,
    } = response.json().await?;

    Ok(List {
        items,
        json_version,
        more_rows,
        returned_rows,
        total_rows,
        transaction_id,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson<A>
where
    A: Attributes,
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
