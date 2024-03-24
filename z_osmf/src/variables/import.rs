use std::marker::PhantomData;
use std::sync::Arc;

use serde::Serialize;
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Endpoint)]
#[endpoint(method = post, path = "/zosmf/variables/rest/1.0/systems/{sysplex}.{system}/actions/import")]
pub(super) struct ImportBuilder<T>
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

    target_type: PhantomData<T>,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct RequestJson<'a> {
    variables_import_file: &'a str,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &ImportBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        variables_import_file: &builder.path,
    })
}
