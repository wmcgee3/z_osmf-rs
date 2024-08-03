use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/workflow/rest/1.0/workflows/{key}/operations/start")]
pub struct WorkflowStartBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    key: Arc<str>,
    #[endpoint(builder_fn = build_body)]
    resolve_conflict_by_using: Option<WorkflowStartResolveVariableConflict>,
    #[endpoint(skip_builder)]
    step_name: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    perform_subsequent: Option<bool>,
    #[endpoint(skip_builder)]
    notification_url: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    target_system_user: Option<Arc<str>>,
    #[endpoint(skip_builder)]
    target_system_password: Option<Arc<str>>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkflowStartResolveVariableConflict {
    OutputFileValue,
    ExistingValue,
    LeaveConflict,
}

impl std::fmt::Display for WorkflowStartResolveVariableConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkflowStartResolveVariableConflict::ExistingValue => "existingValue",
                WorkflowStartResolveVariableConflict::LeaveConflict => "leaveConflict",
                WorkflowStartResolveVariableConflict::OutputFileValue => "outputFileValue",
            }
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestJson<'a> {
    resolve_conflict_by_using: Option<WorkflowStartResolveVariableConflict>,
    step_name: Option<&'a str>,
    perform_subsequent: Option<bool>,
    notification_url: Option<&'a str>,
    target_systemuid: Option<&'a str>,
    target_systempwd: Option<&'a str>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WorkflowStartBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let json = RequestJson {
        resolve_conflict_by_using: builder.resolve_conflict_by_using,
        step_name: builder.step_name.as_deref(),
        perform_subsequent: builder.perform_subsequent,
        notification_url: builder.notification_url.as_deref(),
        target_systemuid: builder.target_system_user.as_deref(),
        target_systempwd: builder.target_system_password.as_deref(),
    };

    request_builder.json(&json)
}
