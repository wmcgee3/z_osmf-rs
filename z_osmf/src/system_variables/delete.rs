use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Endpoint)]
#[endpoint(method = delete, path = "/zosmf/variables/rest/1.0/systems/{sysplex}.{system}")]
pub(super) struct VariableDeleteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    sysplex: Box<str>,
    #[endpoint(path)]
    system: Box<str>,
    #[endpoint(builder_fn = build_body)]
    variable_names: Box<[String]>,

    target_type: PhantomData<T>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &VariableDeleteBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&builder.variable_names)
}
