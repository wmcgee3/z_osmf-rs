use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemId {
    #[default]
    Local,
    Other {
        sysplex: String,
        system: String,
    },
}

impl std::fmt::Display for SystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemId::Local => write!(f, "local"),
            SystemId::Other { sysplex, system } => write!(f, "{}.{}", sysplex, system),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Variable {
    name: Box<str>,
    value: Box<str>,
    description: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Variables {
    inner: Box<[Variable]>,
}

impl TryFromResponse for Variables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        let ResponseJson { variables } = value.json().await?;

        Ok(Variables { inner: variables })
    }
}

impl std::ops::Deref for Variables {
    type Target = [Variable];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/variables/rest/1.0/systems/{system_id}")]
pub struct VariablesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(optional, path)]
    system_id: SystemId,
    #[endpoint(optional, skip_setter, builder_fn = build_names)]
    names: Vec<String>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<'a, T> VariablesBuilder<T>
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
    #[serde(rename = "system-variable-list")]
    variables: Box<[Variable]>,
}

fn build_names<'a, T>(
    request_builder: reqwest::RequestBuilder,
    builder: &VariablesBuilder<T>,
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
