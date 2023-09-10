use std::marker::PhantomData;

use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use zosmf_macros::{Endpoint, Getter};

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct List<A>
where
    A: Attr,
{
    items: Vec<A>,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct ListBuilder<'a, A>
where
    A: Attr,
{
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
    #[endpoint(optional, header = "X-IBM-Attributes", skip_setter)]
    attributes: Option<Attrs>,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, skip_setter, skip_builder)]
    attrs: PhantomData<A>,
}

impl<'a, A> ListBuilder<'a, A>
where
    A: Attr + for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> ListBuilder<'a, Base> {
        ListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            migrated_recall: self.migrated_recall,
            attrs: PhantomData,
        }
    }

    pub fn attributes_member(self) -> ListBuilder<'a, Member> {
        ListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Member),
            migrated_recall: self.migrated_recall,
            attrs: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<List<A>> {
        let response = self.get_response().await?;

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
        })
    }
}

pub enum Base {
    FixedOrVariable { member: String },
    Unformatted { member: String },
}

#[derive(Getter)]
pub struct Member {
    member: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MigratedRecall {
    Error,
    NoWait,
    Wait,
}

impl From<MigratedRecall> for HeaderValue {
    fn from(val: MigratedRecall) -> HeaderValue {
        match val {
            MigratedRecall::Error => "error",
            MigratedRecall::NoWait => "nowait",
            MigratedRecall::Wait => "wait",
        }
        .try_into()
        .unwrap()
    }
}

pub trait Attr {}
impl Attr for Base {}
impl Attr for Member {}

#[derive(Clone, Copy, Debug)]
enum Attrs {
    Base,
    Member,
}

impl From<Attrs> for HeaderValue {
    fn from(val: Attrs) -> HeaderValue {
        match val {
            Attrs::Base => "base",
            Attrs::Member => "member",
        }
        .try_into()
        .unwrap()
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
