use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::get_transaction_id;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct FileGetExtraAttributes {
    name: Box<str>,
    apf_authorized: bool,
    program_controlled: bool,
    shared_address_space: bool,
    shared_library: bool,
    transaction_id: Box<str>,
}

impl FileGetExtraAttributes {
    pub(super) async fn new(
        core: &Arc<ClientCore>,
        path: &str,
    ) -> Result<Self, crate::error::Error> {
        Ok(FileExtraAttributesBuilder::new(core.clone(), path)
            .build()
            .await?)
    }
}

impl TryFromResponse for FileGetExtraAttributes {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let transaction_id = get_transaction_id(&value)?;

        let json: ResponseJson = value.json().await?;

        if let [name, a, p, s, l] = &json.stdout[..] {
            let apf_authorized = a.ends_with("YES");
            let program_controlled = p.ends_with("YES");
            let shared_address_space = s.ends_with("YES");
            let shared_library = l.ends_with("YES");

            Ok(FileGetExtraAttributes {
                name: name.clone(),
                apf_authorized,
                program_controlled,
                shared_address_space,
                shared_library,
                transaction_id,
            })
        } else {
            Err(crate::error::Error::Custom(
                format!("invalid return value format: {:?}", json.stdout).into(),
            ))
        }
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileResetExtraAttributesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_reset_body)]
    apf_authorized: bool,
    #[endpoint(optional, skip_builder)]
    shared_library: bool,
    #[endpoint(optional, skip_builder)]
    program_controlled: bool,
    #[endpoint(optional, skip_builder)]
    shared_address_space: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub struct FileSetExtraAttributesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, builder_fn = build_set_body)]
    apf_authorized: bool,
    #[endpoint(optional, skip_builder)]
    shared_library: bool,
    #[endpoint(optional, skip_builder)]
    program_controlled: bool,
    #[endpoint(optional, skip_builder)]
    shared_address_space: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
struct FileExtraAttributesBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(optional, skip_setter, builder_fn = build_get_body)]
    target_type: PhantomData<T>,
}

#[derive(Serialize)]
struct RequestJson {
    request: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reset: Option<Box<str>>,
}

#[derive(Deserialize)]
struct ResponseJson {
    stdout: Box<[Box<str>]>,
}

fn build_get_body<T>(
    request_builder: reqwest::RequestBuilder,
    _: &FileExtraAttributesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "extattr",
        set: None,
        reset: None,
    })
}

fn build_reset_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileResetExtraAttributesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let mut reset = Vec::new();

    if builder.apf_authorized {
        reset.push('a');
    }
    if builder.shared_library {
        reset.push('l');
    }
    if builder.program_controlled {
        reset.push('p');
    }
    if builder.shared_address_space {
        reset.push('s');
    }
    let reset = Some(reset.into_iter().collect::<String>().into());

    request_builder.json(&RequestJson {
        request: "extattr",
        set: None,
        reset,
    })
}

fn build_set_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &FileSetExtraAttributesBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let mut set = Vec::new();

    if builder.apf_authorized {
        set.push('a');
    }
    if builder.shared_library {
        set.push('l');
    }
    if builder.program_controlled {
        set.push('p');
    }
    if builder.shared_address_space {
        set.push('s');
    }
    let set = Some(set.into_iter().collect::<String>().into());

    request_builder.json(&RequestJson {
        request: "extattr",
        set,
        reset: None,
    })
}
