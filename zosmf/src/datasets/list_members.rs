use std::marker::PhantomData;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use zosmf_macros::{Endpoint, Getter};

use crate::datasets::utils::MigratedRecall;
use crate::utils::{de_optional_y_n, ser_optional_y_n};

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberList<T> {
    items: T,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MembersBase {
    Undefined(Vec<MemberUndefined>),
    FixedOrVariable(Vec<MemberFixedOrVariable>),
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberFixedOrVariable {
    #[serde(rename = "member")]
    name: String,
    #[serde(rename = "vers")]
    version: Option<i32>,
    #[serde(rename = "mod")]
    modification_level: Option<i32>,
    #[serde(rename = "c4date")]
    creation_year: Option<i32>,
    #[serde(rename = "m4date")]
    modification_year: Option<i32>,
    #[serde(rename = "cnorc")]
    current_number_of_records: Option<i32>,
    #[serde(rename = "inorc")]
    initial_number_of_records: Option<i32>,
    #[serde(rename = "mnorc")]
    modified_number_of_records: Option<i32>,
    #[serde(rename = "mtime")]
    modified_time: Option<String>,
    #[serde(rename = "msec")]
    modified_seconds: Option<String>,
    user: Option<String>,
    #[serde(
        default,
        rename = "sclm",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    modified_by_sclm: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberUndefined {
    #[serde(rename = "member")]
    name: String,
    #[serde(rename = "ac")]
    authorization_code: String,
    amode: Option<String>,
    #[serde(rename = "attr")]
    attributes: Option<String>,
    rmode: Option<String>,
    size: Option<String>,
    ttr: Option<String>,
    ssi: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberName {
    #[serde(rename = "member")]
    name: String,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct MemberListBuilder<'a, T> {
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    dataset_name: String,

    #[endpoint(optional, query = "start")]
    start: Option<String>,
    #[endpoint(optional, query = "pattern")]
    pattern: Option<String>,
    #[endpoint(optional, header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_attributes")]
    attributes: Option<Attrs>,
    #[endpoint(optional, skip_setter, skip_builder)]
    include_total: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, skip_setter, skip_builder)]
    attrs: PhantomData<T>,
}

impl<'a, T> MemberListBuilder<'a, T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> MemberListBuilder<'a, MembersBase> {
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
            attrs: PhantomData,
        }
    }

    pub fn attributes_member(self) -> MemberListBuilder<'a, Vec<MemberName>> {
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
            attrs: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<MemberList<T>> {
        let response = self.get_response().await?;

        let ResponseJson {
            items,
            returned_rows,
            more_rows,
            total_rows,
            json_version,
        } = response.json().await?;

        Ok(MemberList {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
        })
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
    items: T,
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
    member_list_builder: &MemberListBuilder<T>,
) -> RequestBuilder {
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
