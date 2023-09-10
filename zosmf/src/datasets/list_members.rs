use std::marker::PhantomData;

use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use zosmf_macros::Endpoint;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListMembers<A>
where
    A: Attributes,
{
    pub items: Vec<A>,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct ListMembersBuilder<'a, A>
where
    A: Attributes,
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
    #[endpoint(optional, skip_setter, skip_builder)]
    attributes: PhantomData<A>,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
}

pub trait Attributes {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Base {}

impl Attributes for Base {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    pub member: String,
}

impl Attributes for Member {}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MigratedRecall {
    Error,
    NoWait,
    Wait,
}

impl Into<HeaderValue> for MigratedRecall {
    fn into(self) -> HeaderValue {
        match self {
            MigratedRecall::Error => "error",
            MigratedRecall::NoWait => "nowait",
            MigratedRecall::Wait => "wait",
        }
        .try_into()
        .unwrap()
    }
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
    more_rows: Option<i32>,
    #[serde(default)]
    total_rows: Option<i32>,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}
