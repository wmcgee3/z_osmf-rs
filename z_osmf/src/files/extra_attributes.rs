pub mod reset;
pub mod set;

use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::restfiles::get_transaction_id;
use crate::{ClientCore, Error};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FileExtraAttributeList {
    name: Box<str>,
    apf_authorized: bool,
    program_controlled: bool,
    shared_address_space: bool,
    shared_library: bool,
    transaction_id: Box<str>,
}

impl TryFromResponse for FileExtraAttributeList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        let json: ResponseJson = value.json().await?;

        if let [name, a, p, s, l] = &json.stdout[..] {
            let apf_authorized = a.ends_with("YES");
            let program_controlled = p.ends_with("YES");
            let shared_address_space = s.ends_with("YES");
            let shared_library = l.ends_with("YES");

            Ok(FileExtraAttributeList {
                name: name.clone(),
                apf_authorized,
                program_controlled,
                shared_address_space,
                shared_library,
                transaction_id,
            })
        } else {
            Err(Error::InvalidFormat(json.stdout))
        }
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/fs{path}")]
pub(crate) struct FileExtraAttributeListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,

    #[endpoint(builder_fn = build_body)]
    target_type: PhantomData<T>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    _: &FileExtraAttributeListBuilder<T>,
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
