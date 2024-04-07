use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Create {
    description: Box<str>,
    id: Box<str>,
    key: Box<str>,
    vendor: Box<str>,
    version: Box<str>,
}

impl TryFromResponse for Create {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = post, path = "/zosmf/workflow/rest/1.0/workflows")]
pub struct CreateBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(builder_fn = build_body)]
    name: Box<str>,
    definition_file: Box<str>,
    system: Box<str>,
    owner: Box<str>,

    #[endpoint(skip_builder)]
    definition_file_system: Option<Box<str>>,
    #[endpoint(skip_builder)]
    variable_file: Option<Box<str>>,
    #[endpoint(skip_builder)]
    variables: Option<Vec<(Box<str>, Box<str>)>>,
    #[endpoint(skip_builder)]
    conflict_resolution: Option<Box<str>>,
    #[endpoint(skip_builder)]
    archive_owner: Option<Box<str>>,
    #[endpoint(skip_builder)]
    comments: Option<Box<str>>,
    #[endpoint(skip_builder)]
    assign_to_owner: Option<bool>,
    #[endpoint(skip_builder)]
    access_type: Option<Box<str>>,
    #[endpoint(skip_builder)]
    account_info: Option<Box<str>>,
    #[endpoint(skip_builder)]
    job_statement: Option<Box<str>>,
    #[endpoint(skip_builder)]
    delete_completed: Option<bool>,
    #[endpoint(skip_builder)]
    output_directory: Option<Box<str>>,
    #[endpoint(skip_builder)]
    delete_on_completion: Option<bool>,
    #[endpoint(skip_builder)]
    system_uid: Option<Box<str>>,
    #[endpoint(skip_builder)]
    system_password: Option<Box<str>>,

    target_type: PhantomData<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Variable {
    name: Box<str>,
    value: Box<str>,
}

impl Variable {
    pub fn new(name: &str, value: &str) -> Self {
        Variable {
            name: name.into(),
            value: value.into(),
        }
    }
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
    variables: Option<&'a [(Box<str>, Box<str>)]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolve_global_conflict_by_using: Option<&'a str>,
    system: &'a str,
    owner: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_archive_s_a_f_i_d: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assign_to_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<&'a str>,
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
    builder: &CreateBuilder<T>,
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
        variable_input_file: builder.variable_file.as_deref(),
        variables: builder.variables.as_deref(),
        resolve_global_conflict_by_using: builder.conflict_resolution.as_deref(),
        workflow_archive_s_a_f_i_d: builder.archive_owner.as_deref(),
        comments: builder.comments.as_deref(),
        assign_to_owner: builder.assign_to_owner,
        access_type: builder.access_type.as_deref(),
        account_info: builder.account_info.as_deref(),
        job_statement: builder.job_statement.as_deref(),
        delete_completed_jobs: builder.delete_completed,
        jobs_output_directory: builder.output_directory.as_deref(),
        auto_delete_on_completion: builder.delete_on_completion,
        target_systemuid: builder.system_uid.as_deref(),
        target_systempwd: builder.system_password.as_deref(),
    })
}
