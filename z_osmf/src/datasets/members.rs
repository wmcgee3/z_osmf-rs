use std::marker::PhantomData;
use std::sync::Arc;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

use super::{de_optional_y_n, ser_optional_y_n, DatasetMigratedRecall};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MemberAttributesBase {
    #[serde(rename = "member")]
    name: Box<str>,
    #[getter(copy)]
    #[serde(default, rename = "vers")]
    version: Option<i32>,
    #[getter(copy)]
    #[serde(default, rename = "mod")]
    modification_level: Option<i32>,
    #[getter(copy)]
    #[serde(default, rename = "c4date")]
    creation_date: Option<NaiveDate>,
    #[getter(copy)]
    #[serde(default, rename = "m4date")]
    modification_date: Option<NaiveDate>,
    #[getter(copy)]
    #[serde(default, rename = "cnorc")]
    current_number_of_records: Option<i32>,
    #[getter(copy)]
    #[serde(default, rename = "inorc")]
    initial_number_of_records: Option<i32>,
    #[getter(copy)]
    #[serde(default, rename = "mnorc")]
    modified_number_of_records: Option<i32>,
    #[serde(default, rename = "mtime")]
    modified_time: Option<Box<str>>,
    #[serde(default, rename = "msec")]
    modified_seconds: Option<Box<str>>,
    #[serde(default)]
    user: Option<Box<str>>,
    #[getter(copy)]
    #[serde(
        default,
        rename = "sclm",
        deserialize_with = "de_optional_y_n",
        serialize_with = "ser_optional_y_n"
    )]
    modified_by_sclm: Option<bool>,
    #[serde(default, rename = "ac")]
    authorization_code: Option<Box<str>>,
    #[serde(default)]
    amode: Option<Box<str>>,
    #[serde(default, rename = "attr")]
    attributes: Option<Box<str>>,
    #[serde(default)]
    rmode: Option<Box<str>>,
    #[serde(default)]
    size: Option<Box<str>>,
    #[serde(default)]
    ttr: Option<Box<str>>,
    #[serde(default)]
    ssi: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MemberAttributesName {
    #[serde(rename = "member")]
    name: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MemberList<T> {
    items: Box<[T]>,
    #[getter(copy)]
    json_version: i32,
    #[getter(copy)]
    more_rows: Option<bool>,
    #[getter(copy)]
    returned_rows: i32,
    #[getter(copy)]
    total_rows: Option<i32>,
}

impl<T> TryFromResponse for MemberList<T>
where
    T: for<'de> Deserialize<'de>,
{
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
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

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset}/member")]
pub struct MemberListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    dataset: Box<str>,
    #[endpoint(query = "start")]
    start: Option<Box<str>>,
    #[endpoint(query = "pattern")]
    pattern: Option<Box<str>>,
    #[endpoint(header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(skip_setter, builder_fn = build_attributes)]
    attributes: Option<Attrs>,
    #[endpoint(skip_builder)]
    include_total: Option<bool>,
    #[endpoint(header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<DatasetMigratedRecall>,

    target_type: PhantomData<T>,
}

impl<T> MemberListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn attributes_base(self) -> MemberListBuilder<MemberList<MemberAttributesBase>> {
        MemberListBuilder {
            core: self.core,
            dataset: self.dataset,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            include_total: self.include_total,
            migrated_recall: self.migrated_recall,
            target_type: PhantomData,
        }
    }

    pub fn attributes_member(self) -> MemberListBuilder<MemberList<MemberAttributesName>> {
        MemberListBuilder {
            core: self.core,
            dataset: self.dataset,
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
        (None, Some(true)) => request_builder.header(key, "member,total"),
        (Some(attr), total) => request_builder.header(
            key,
            format!(
                "{}{}",
                attr,
                if *total == Some(true) { ",total" } else { "" }
            ),
        ),
        _ => request_builder,
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
            .get("https://test.com/zosmf/restfiles/ds/NOTSYS1.PROCLIB/member")
            .build()
            .unwrap();

        let list_members = zosmf
            .datasets()
            .members("NOTSYS1.PROCLIB")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", list_members)
        );
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/ds/NOTSYS1.PROCLIB/member")
            .header("X-IBM-Attributes", "base")
            .build()
            .unwrap();

        let list_members_base = zosmf
            .datasets()
            .members("NOTSYS1.PROCLIB")
            .attributes_base()
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", list_members_base)
        );
    }
}
