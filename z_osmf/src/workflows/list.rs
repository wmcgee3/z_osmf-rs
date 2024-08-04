use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

use super::{WorkflowAccess, WorkflowStatus};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowCategory {
    Configuration,
    General,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowInfo {
    #[serde(rename = "workflowName")]
    name: Arc<str>,
    #[serde(rename = "workflowKey")]
    key: Arc<str>,
    #[serde(rename = "workflowDescription")]
    description: Arc<str>,
    #[serde(rename = "workflowID")]
    id: Arc<str>,
    #[serde(rename = "workflowVersion")]
    version: Arc<str>,
    #[serde(rename = "workflowDefinitionFileMD5Value")]
    definition_file_hash: Arc<str>,
    #[serde(rename = "instanceURI")]
    instance_uri: Arc<str>,
    owner: Arc<str>,
    vendor: Arc<str>,
    #[getter(copy)]
    access: WorkflowAccess,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowList {
    items: Arc<[WorkflowInfo]>,
}

impl TryFromResponse for WorkflowList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let items = value.json::<ResponseJson>().await?.workflows;

        Ok(WorkflowList { items })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/workflow/rest/1.0/workflows")]
pub struct WorkflowListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(query = "workflowName")]
    name: Option<Arc<str>>,
    #[endpoint(query = "category")]
    category: Option<WorkflowCategory>,
    #[endpoint(query = "system")]
    system: Option<Arc<str>>,
    #[endpoint(query = "statusName")]
    status: Option<WorkflowStatus>,
    #[endpoint(query = "owner")]
    owner: Option<Arc<str>>,
    #[endpoint(query = "vendor")]
    vendor: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

#[derive(Deserialize)]
struct ResponseJson {
    workflows: Arc<[WorkflowInfo]>,
}
