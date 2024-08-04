use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::RequestJson;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileExtraAttributesResetBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Arc<str>,
    #[endpoint(builder_fn = build_body)]
    apf_authorized: Option<bool>,
    #[endpoint(skip_builder)]
    shared_library: Option<bool>,
    #[endpoint(skip_builder)]
    program_controlled: Option<bool>,
    #[endpoint(skip_builder)]
    shared_address_space: Option<bool>,

    target_type: PhantomData<T>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileExtraAttributesResetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let mut reset = Vec::new();

    if builder.apf_authorized == Some(true) {
        reset.push('a');
    }
    if builder.shared_library == Some(true) {
        reset.push('l');
    }
    if builder.program_controlled == Some(true) {
        reset.push('p');
    }
    if builder.shared_address_space == Some(true) {
        reset.push('s');
    }
    let reset = Some(reset.into_iter().collect::<String>().into());

    request_builder.json(&RequestJson {
        request: "extattr",
        set: None,
        reset,
    })
}
