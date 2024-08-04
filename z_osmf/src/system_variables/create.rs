use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewSystemVariable {
    name: Arc<str>,
    value: Arc<str>,
    description: Arc<str>,
}

impl NewSystemVariable {
    pub fn new<N, V, D>(name: N, value: V, description: D) -> Self
    where
        N: std::fmt::Display,
        V: std::fmt::Display,
        D: std::fmt::Display,
    {
        NewSystemVariable {
            name: name.to_string().into(),
            value: value.to_string().into(),
            description: description.to_string().into(),
        }
    }
}

#[derive(Endpoint)]
#[endpoint(method = post, path = "/zosmf/variables/rest/1.0/systems/{sysplex}.{system}")]
pub(crate) struct VariableCreateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    sysplex: Arc<str>,
    #[endpoint(path)]
    system: Arc<str>,
    #[endpoint(builder_fn = build_body)]
    new_variables: Arc<[NewSystemVariable]>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    system_variable_list: &'a [NewSystemVariable],
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &VariableCreateBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        system_variable_list: &builder.new_variables,
    })
}
