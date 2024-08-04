use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Info {
    zosmf_saf_realm: Arc<str>,
    zosmf_port: Arc<str>,
    plugins: Arc<[Plugin]>,
    api_version: Arc<str>,
    zos_version: Arc<str>,
    zosmf_version: Arc<str>,
    zosmf_hostname: Arc<str>,
}

impl TryFromResponse for Info {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Plugin {
    #[serde(rename = "pluginVersion")]
    version: Arc<str>,
    #[serde(default, rename = "pluginStatus")]
    status: Option<Arc<str>>,
    #[serde(rename = "pluginDefaultName")]
    default_name: Arc<str>,
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
