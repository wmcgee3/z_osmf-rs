use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArchivedWorkflow {
    name: Arc<str>,
    key: Arc<str>,
    uri: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArchivedWorkflowList {
    items: Arc<[ArchivedWorkflow]>,
}

impl TryFromResponse for ArchivedWorkflowList {
    async fn try_from_response(value: reqwest::Response) -> crate::Result<Self> {
        let json: ResponseJson = value.json().await?;
        let items = json
            .archived_workflows
            .into_iter()
            .map(|workflow| ArchivedWorkflow {
                name: workflow.workflow_name,
                key: workflow.workflow_key,
                uri: workflow.archived_instance_u_r_i,
            })
            .collect();

        Ok(ArchivedWorkflowList { items })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/workflow/rest/1.0/archivedworkflows")]
pub struct ArchivedWorkflowListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(query = "Orderby")]
    order_by: Option<WorkflowOrderBy>,
    #[endpoint(query = "View")]
    view: Option<WorkflowView>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WorkflowOrderBy {
    Asc,
    Desc,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum WorkflowView {
    Domain,
    #[default]
    User,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    archived_workflows: Vec<ResponseArchivedWorkflow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseArchivedWorkflow {
    workflow_name: Arc<str>,
    workflow_key: Arc<str>,
    archived_instance_u_r_i: Arc<str>,
}
