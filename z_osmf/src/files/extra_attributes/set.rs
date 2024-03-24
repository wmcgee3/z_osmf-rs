use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::RequestJson;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct SetBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
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
    builder: &SetBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let mut set = Vec::new();

    if builder.apf_authorized == Some(true) {
        set.push('a');
    }
    if builder.shared_library == Some(true) {
        set.push('l');
    }
    if builder.program_controlled == Some(true) {
        set.push('p');
    }
    if builder.shared_address_space == Some(true) {
        set.push('s');
    }
    let set = Some(set.into_iter().collect::<String>().into());

    request_builder.json(&RequestJson {
        request: "extattr",
        set,
        reset: None,
    })
}
