use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};
use crate::datasets::MigratedRecall;
use crate::error::Error;
use crate::utils::{de_optional_y_n, ser_optional_y_n};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberList<T> {
    pub items: Box<[T]>,
    pub json_version: i32,
    pub more_rows: Option<bool>,
    pub returned_rows: i32,
    pub total_rows: Option<i32>,
}

impl<T> TryFromResponse for MemberList<T>
where
    T: for<'de> Deserialize<'de>,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let ResponseJson {
            items,
            returned_rows,
            more_rows,
            total_rows,
            json_version,
        } = value.json().await?;

        Ok(MemberList {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberBase {
    #[serde(rename = "member")]
    pub name: Box<str>,
    #[serde(default, rename = "vers")]
    pub version: Option<i32>,
    #[serde(default, rename = "mod")]
    pub modification_level: Option<i32>,
    #[serde(default, rename = "c4date")]
    pub creation_date: Option<Box<str>>,
    #[serde(default, rename = "m4date")]
    pub modification_date: Option<Box<str>>,
    #[serde(default, rename = "cnorc")]
    pub current_number_of_records: Option<i32>,
    #[serde(default, rename = "inorc")]
    pub initial_number_of_records: Option<i32>,
    #[serde(default, rename = "mnorc")]
    pub modified_number_of_records: Option<i32>,
    #[serde(default, rename = "mtime")]
    pub modified_time: Option<Box<str>>,
    #[serde(default, rename = "msec")]
    pub modified_seconds: Option<Box<str>>,
    #[serde(default)]
    pub user: Option<Box<str>>,
    #[serde(
        default,
        rename = "sclm",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    pub modified_by_sclm: Option<bool>,
    #[serde(default, rename = "ac")]
    pub authorization_code: Option<Box<str>>,
    #[serde(default)]
    pub amode: Option<Box<str>>,
    #[serde(default, rename = "attr")]
    pub attributes: Option<Box<str>>,
    #[serde(default)]
    pub rmode: Option<Box<str>>,
    #[serde(default)]
    pub size: Option<Box<str>>,
    #[serde(default)]
    pub ttr: Option<Box<str>>,
    #[serde(default)]
    pub ssi: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberName {
    #[serde(rename = "member")]
    pub name: Box<str>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct MemberListBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, query = "start")]
    start: Option<Box<str>>,
    #[endpoint(optional, query = "pattern")]
    pattern: Option<Box<str>>,
    #[endpoint(optional, header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_attributes")]
    attributes: Option<Attrs>,
    #[endpoint(optional, skip_setter, skip_builder)]
    include_total: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> MemberListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn attributes_base(self) -> MemberListBuilder<MemberList<MemberBase>> {
        MemberListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            include_total: self.include_total,
            migrated_recall: self.migrated_recall,
            target_type: PhantomData,
        }
    }

    pub fn attributes_member(self) -> MemberListBuilder<MemberList<MemberName>> {
        MemberListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Member),
            include_total: self.include_total,
            migrated_recall: self.migrated_recall,
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Attrs {
    Base,
    Member,
}

impl std::fmt::Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Attrs::Base => "base",
                Attrs::Member => "member",
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
    request_builder: reqwest::RequestBuilder,
    member_list_builder: &MemberListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let MemberListBuilder {
        attributes,
        include_total,
        ..
    } = member_list_builder;
    let key = "X-IBM-Attributes";

    match (attributes, include_total) {
        (None, false) => request_builder,
        (None, true) => request_builder.header(key, "member,total"),
        (Some(attr), total) => request_builder.header(
            key,
            format!("{}{}", attr, if *total { ",total" } else { "" }),
        ),
    }
}
