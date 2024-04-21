use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/variables/rest/1.0/systems/{sysplex}.{system}/actions/export")]
pub struct VariableExportBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    sysplex: Box<str>,
    #[endpoint(path)]
    system: Box<str>,
    #[endpoint(builder_fn = build_body)]
    path: Box<str>,
    overwrite: Option<bool>,

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    variables_export_file: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<bool>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &VariableExportBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        variables_export_file: &builder.path,
        overwrite: builder.overwrite,
    })
}
