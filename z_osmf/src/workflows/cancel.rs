use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowCancel {
    inner: Box<str>,
}

impl std::ops::Deref for WorkflowCancel {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowCancel {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        let ResponseJson { workflow_name } = value.json().await?;

        Ok(WorkflowCancel {
            inner: workflow_name,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/workflow/rest/1.0/workflows/{key}/operations/cancel")]
pub(super) struct WorkflowCancelBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    key: Box<str>,

    target_type: PhantomData<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    workflow_name: Box<str>,
}
