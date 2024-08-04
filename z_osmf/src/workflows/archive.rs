use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowArchive {
    key: Arc<str>,
}

impl TryFromResponse for WorkflowArchive {
    async fn try_from_response(value: reqwest::Response) -> crate::Result<Self> {
        let json: ResponseJson = value.json().await?;

        Ok(WorkflowArchive {
            key: json.workflow_key,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/workflow/rest/1.0/workflows/{key}/operations/archive")]
pub struct WorkflowArchiveBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    key: Arc<str>,

    target_type: PhantomData<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    workflow_key: Arc<str>,
}
