use std::marker::PhantomData;
use std::sync::Arc;

use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::WorkflowType;

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
