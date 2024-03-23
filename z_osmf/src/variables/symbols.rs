use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Symbol {
    name: Box<str>,
    value: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Symbols {
    inner: Box<[Symbol]>,
}

impl TryFromResponse for Symbols {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        let ResponseJson { symbols } = value.json().await?;

        Ok(Symbols { inner: symbols })
    }
}

impl std::ops::Deref for Symbols {
    type Target = [Symbol];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/variables/rest/1.0/systems/local?source=symbol")]
pub struct SymbolsBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, skip_setter, builder_fn = build_names)]
    names: Vec<String>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<T> SymbolsBuilder<T>
where
    T: TryFromResponse,
{
    pub fn name<V>(self, value: V) -> Self
    where
        V: ToString,
    {
        let mut new = self;
        new.names.push(value.to_string());

        new
    }

    pub fn names<V>(self, value: &[V]) -> Self
    where
        V: ToString,
    {
        let mut new = self;
        new.names.extend(value.iter().map(|v| v.to_string()));

        new
    }
}

#[derive(Deserialize)]
struct ResponseJson {
    #[serde(rename = "system-symbol-list")]
    symbols: Box<[Symbol]>,
}

fn build_names<'a, T>(
    request_builder: reqwest::RequestBuilder,
    builder: &SymbolsBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let query: Box<[_]> = builder
        .names
        .iter()
        .map(|name| ("var-name", name))
        .collect();

    request_builder.query(&query)
}
