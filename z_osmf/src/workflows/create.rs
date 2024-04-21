use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::WorkflowAccess;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowCreate {
    description: Box<str>,
    id: Box<str>,
    key: Box<str>,
    vendor: Box<str>,
    version: Box<str>,
}

impl TryFromResponse for WorkflowCreate {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/workflow/rest/1.0/workflows")]
pub struct WorkflowCreateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    name: Box<str>,
    definition_file: Box<str>,
    system: Box<str>,
    owner: Box<str>,

    definition_file_system: Option<Box<str>>,
    variable_input_file: Option<Box<str>>,
    variables: Option<Box<[WorkflowVariableOverride]>>,
    resolve_global_conflict_by_using: Option<WorkflowVariableResolveConflict>,
    archive_saf_id: Option<Box<str>>,
    comments: Option<Box<str>>,
    assign_to_owner: Option<bool>,
    access_type: Option<WorkflowAccess>,
    account_info: Option<Box<str>>,
    job_statement: Option<Box<str>>,
    delete_completed_jobs: Option<bool>,
    jobs_output_directory: Option<Box<str>>,
    auto_delete_on_completion: Option<bool>,
    target_system_uid: Option<Box<str>>,
    target_system_password: Option<Box<str>>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowVariableOverride {
    name: Box<str>,
    value: Box<str>,
}

impl WorkflowVariableOverride {
    pub fn new(name: &str, value: &str) -> Self {
        WorkflowVariableOverride {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowVariableResolveConflict {
    #[default]
    Global,
    Input,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestJson<'a> {
    workflow_name: &'a str,
    workflow_definition_file: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_definition_file_system: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_input_file: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<&'a [WorkflowVariableOverride]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_global_conflict_by_using: Option<&'a WorkflowVariableResolveConflict>,
    system: &'a str,
    owner: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_archive_s_a_f_i_d: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_to_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<&'a WorkflowAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_info: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_statement: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_completed_jobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jobs_output_directory: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete_on_completion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_systemuid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_systempwd: Option<&'a str>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WorkflowCreateBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        workflow_name: &builder.name,
        workflow_definition_file: &builder.definition_file,
        system: &builder.system,
        owner: &builder.owner,
        workflow_definition_file_system: builder.definition_file_system.as_deref(),
        variable_input_file: builder.variable_input_file.as_deref(),
        variables: builder.variables.as_deref(),
        resolve_global_conflict_by_using: builder.resolve_global_conflict_by_using.as_ref(),
        workflow_archive_s_a_f_i_d: builder.archive_saf_id.as_deref(),
        comments: builder.comments.as_deref(),
        assign_to_owner: builder.assign_to_owner,
        access_type: builder.access_type.as_ref(),
        account_info: builder.account_info.as_deref(),
        job_statement: builder.job_statement.as_deref(),
        delete_completed_jobs: builder.delete_completed_jobs,
        jobs_output_directory: builder.jobs_output_directory.as_deref(),
        auto_delete_on_completion: builder.auto_delete_on_completion,
        target_systemuid: builder.target_system_uid.as_deref(),
        target_systempwd: builder.target_system_password.as_deref(),
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::workflows::create::WorkflowAccess;

    #[test]
    fn example() -> anyhow::Result<()> {
        let zosmf = get_zosmf();

        let json: serde_json::Value = serde_json::from_str(
            r#"
            {
                "workflowName":"AutomationExample",
                "workflowDefinitionFile":"/usr/lpp/zosmf/samples/workflow_sample_automation.xml",
                "system":"SY1",
                "owner":"zosmfad",
                "assignToOwner" :true,
                "accessType": "Restricted",
                "deleteCompletedJobs" : true,
                "autoDeleteOnCompletion" : true
            }
"#,
        )?;

        let manual_request = zosmf
            .core
            .client
            .post("https://test.com/zosmf/workflow/rest/1.0/workflows")
            .json(&json)
            .build()?;

        let create = zosmf
            .workflows()
            .create(
                "AutomationExample",
                "/usr/lpp/zosmf/samples/workflow_sample_automation.xml",
                "SY1",
                "zosmfad",
            )
            .assign_to_owner(true)
            .access_type(WorkflowAccess::Restricted)
            .delete_completed_jobs(true)
            .auto_delete_on_completion(true)
            .get_request()?;

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", create));

        assert_eq!(manual_request.json(), create.json());

        Ok(())
    }
}
