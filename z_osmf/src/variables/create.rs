use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewVariable {
    name: String,
    value: String,
    description: String,
}

impl NewVariable {
    pub fn new<N, V, D>(name: N, value: V, description: D) -> Self
    where
        N: ToString,
        V: ToString,
        D: ToString,
    {
        NewVariable {
            name: name.to_string(),
            value: value.to_string(),
            description: description.to_string(),
        }
    }

    pub fn builder<N, V>(name: N, value: V) -> NewVariableBuilder
    where
        N: ToString,
        V: ToString,
    {
        NewVariableBuilder::new(name, value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewVariableBuilder {
    name: String,
    value: String,
    description: String,
}

impl NewVariableBuilder {
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: ToString,
        V: ToString,
    {
        NewVariableBuilder {
            name: name.to_string(),
            value: value.to_string(),
            description: String::new(),
        }
    }

    pub fn description<V>(self, value: V) -> Self
    where
        V: ToString,
    {
        let mut new = self;
        new.description = value.to_string();

        new
    }

    pub fn build(self) -> NewVariable {
        NewVariable {
            name: self.name,
            value: self.value,
            description: self.description,
        }
    }
}

#[derive(Endpoint)]
#[endpoint(method = post, path = "/zosmf/variables/rest/1.0/systems/{sysplex}.{system}")]
pub(super) struct CreateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    sysplex: Box<str>,
    #[endpoint(path)]
    system: Box<str>,
    #[endpoint(builder_fn = build_body)]
    new_variables: Box<[NewVariable]>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    system_variable_list: &'a [NewVariable],
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &CreateBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        system_variable_list: &builder.new_variables,
    })
}
