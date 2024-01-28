use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;
use z_osmf_macros::{Endpoint, Getters};

use crate::datasets::MigratedRecall;
use crate::error::Error;
use crate::utils::{de_optional_y_n, ser_optional_y_n};

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct MemberList<T> {
    items: Box<[T]>,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
}

impl<T> TryFrom<reqwest::Response> for MemberList<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Error = Error;

    fn try_from(value: reqwest::Response) -> Result<Self, Self::Error> {
        let ResponseJson {
            items,
            returned_rows,
            more_rows,
            total_rows,
            json_version,
        } = Handle::current().block_on(value.json())?;

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
#[serde(untagged)]
pub enum MembersBase {
    Undefined {
        #[serde(rename = "member")]
        name: Box<str>,
        #[serde(rename = "ac")]
        authorization_code: Box<str>,
        amode: Option<Box<str>>,
        #[serde(rename = "attr")]
        attributes: Option<Box<str>>,
        rmode: Option<Box<str>>,
        size: Option<Box<str>>,
        ttr: Option<Box<str>>,
        ssi: Option<Box<str>>,
    },
    FixedOrVariable {
        #[serde(rename = "member")]
        name: Box<str>,
        #[serde(rename = "vers")]
        version: i32,
        #[serde(rename = "mod")]
        modification_level: i32,
        #[serde(rename = "c4date")]
        creation_date: Box<str>,
        #[serde(rename = "m4date")]
        modification_date: Box<str>,
        #[serde(rename = "cnorc")]
        current_number_of_records: i32,
        #[serde(rename = "inorc")]
        initial_number_of_records: i32,
        #[serde(rename = "mnorc")]
        modified_number_of_records: i32,
        #[serde(rename = "mtime")]
        modified_time: Box<str>,
        #[serde(rename = "msec")]
        modified_seconds: Box<str>,
        user: Box<str>,
        #[serde(
            default,
            rename = "sclm",
            deserialize_with = "de_optional_y_n",
            serialize_with = "ser_optional_y_n"
        )]
        modified_by_sclm: Option<bool>,
    },
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct MemberName {
    #[serde(rename = "member")]
    name: Box<str>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct MemberListBuilder<T> {
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
    attributes_marker: PhantomData<T>,
}

impl<T> MemberListBuilder<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> MemberListBuilder<MembersBase> {
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
            attributes_marker: PhantomData,
        }
    }

    pub fn attributes_member(self) -> MemberListBuilder<Box<[MemberName]>> {
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
            attributes_marker: PhantomData,
        }
    }

    pub async fn build(self) -> Result<MemberList<T>, Error> {
        let response = self.get_response().await?;

        response.try_into()
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
) -> reqwest::RequestBuilder {
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
