use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/workflow/rest/1.0/{workflow_type}/{key}")]
pub struct WorkflowDeleteBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, skip_setter)]
    workflow_type: WorkflowType,
    #[endpoint(path)]
    key: Arc<str>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub(super) enum WorkflowType {
    ArchivedWorkflows,
    Workflows,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkflowType::ArchivedWorkflows => "archivedworkflows",
                WorkflowType::Workflows => "workflows",
            }
        )
    }
}
