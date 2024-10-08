use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemSymbol {
    name: Arc<str>,
    value: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemSymbolList {
    inner: Arc<[SystemSymbol]>,
}

impl TryFromResponse for SystemSymbolList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let ResponseJson { symbols } = value.json().await?;

        Ok(SystemSymbolList { inner: symbols })
    }
}

impl std::ops::Deref for SystemSymbolList {
    type Target = [SystemSymbol];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/variables/rest/1.0/systems/local?source=symbol")]
pub struct SystemSymbolListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(skip_setter, builder_fn = build_names)]
    names: Option<Vec<String>>,

    target_type: PhantomData<T>,
}

impl<T> SystemSymbolListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn name<V>(self, value: V) -> Self
    where
        V: std::fmt::Display,
    {
        let mut new = self;
        match new.names {
            Some(ref mut names) => names.push(value.to_string()),
            None => new.names = Some(vec![value.to_string()]),
        }

        new
    }

    pub fn names<V>(self, value: &[V]) -> Self
    where
        V: std::fmt::Display,
    {
        let mut new = self;
        match new.names {
            Some(ref mut names) => names.extend(value.iter().map(|v| v.to_string())),
            None => new.names = Some(value.iter().map(|v| v.to_string()).collect()),
        }

        new
    }
}

#[derive(Deserialize)]
struct ResponseJson {
    #[serde(rename = "system-symbol-list")]
    symbols: Arc<[SystemSymbol]>,
}

fn build_names<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &SystemSymbolListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let query: Arc<[_]> = builder
        .names
        .iter()
        .map(|name| ("var-name", name))
        .collect();

    request_builder.query(&query)
}
