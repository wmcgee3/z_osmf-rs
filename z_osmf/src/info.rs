use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Info {
    zosmf_saf_realm: Box<str>,
    zosmf_port: Box<str>,
    plugins: Box<[Plugin]>,
    api_version: Box<str>,
    zos_version: Box<str>,
    zosmf_version: Box<str>,
    zosmf_hostname: Box<str>,
}

impl TryFromResponse for Info {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Plugin {
    #[serde(rename = "pluginVersion")]
    version: Box<str>,
    #[serde(default, rename = "pluginStatus")]
    status: Option<Box<str>>,
    #[serde(rename = "pluginDefaultName")]
    default_name: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/info")]
pub(crate) struct InfoBuilder<T>
where
    T: TryFromResponse,
{
    core: ClientCore,

    target_type: PhantomData<T>,
}
