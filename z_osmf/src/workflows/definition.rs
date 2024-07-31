use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::ClientCore;

use super::ReturnData;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowDefinition {
    #[serde(rename = "workflowDefaultName")]
    default_name: Option<Box<str>>,
    #[serde(rename = "workflowDescription")]
    description: Box<str>,
    #[serde(rename = "workflowID")]
    id: Box<str>,
    #[serde(rename = "workflowVersion")]
    version: Box<str>,
    vendor: Box<str>,
    #[serde(rename = "workflowDefinitionFileMD5Value")]
    hash: Box<str>,
    is_callable: Option<Box<str>>,
    #[getter(copy)]
    contains_parallel_steps: bool,
    scope: Box<str>,
    jobs_output_directory: Option<Box<str>>,
    category: Box<str>,
    #[serde(rename = "productID")]
    product_id: Option<Box<str>>,
    product_name: Option<Box<str>>,
    product_version: Option<Box<str>>,
    global_variable_group: Option<Box<str>>,
    #[getter(copy)]
    is_instance_variable_without_prefix: bool,
    software_type: Option<Box<str>>,
}

impl TryFromResponse for WorkflowDefinition {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionSteps {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinition,
    steps: Box<[StepCore]>,
}

impl std::ops::Deref for WorkflowDefinitionSteps {
    type Target = WorkflowDefinition;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionSteps {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionStepsVariables {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinitionSteps,
    variables: Box<[VariableDefinition]>,
}

impl std::ops::Deref for WorkflowDefinitionStepsVariables {
    type Target = WorkflowDefinitionSteps;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionStepsVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WorkflowDefinitionVariables {
    #[getter(skip)]
    #[serde(flatten)]
    inner: WorkflowDefinition,
    variables: Box<[VariableDefinition]>,
}

impl std::ops::Deref for WorkflowDefinitionVariables {
    type Target = WorkflowDefinition;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFromResponse for WorkflowDefinitionVariables {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        let text: String = value.text().await?;
        std::fs::write("output.json", &text).unwrap();

        Ok(serde_json::from_str(&text).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepCore {
    name: Box<str>,
    title: Box<str>,
    description: Box<str>,
    prereq_step: Option<Box<[Box<str>]>>,
    optional: bool,
    steps: Option<Box<[StepCore]>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VariableDefinition {
    name: Box<str>,
    scope: Box<str>,
    #[serde(rename = "abstract")]
    short_description: Box<str>,
    category: Box<str>,
    choice: Option<Box<[Box<str>]>>,
    decimal_places: Option<i32>,
    default: Option<Box<str>>,
    description: Box<str>,
    expose_to_user: bool,
    max_length: Option<i32>,
    max_value: Option<Box<str>>,
    min_length: Option<i32>,
    min_value: Option<Box<str>>,
    prompt_at_create: bool,
    regular_expression: Option<Box<str>>,
    required_at_create: bool,
    #[serde(rename = "type")]
    variable_type: Box<str>,
    validation_type: Option<Box<str>>,
    value_must_be_choice: Option<bool>,
    visibility: Box<str>,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/workflow/rest/1.0/workflowDefinition?definitionFilePath={path}")]
pub struct WorkflowDefinitionBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(query = "workflowDefinitionFileSystem")]
    file_system: Option<Box<str>>,
    #[endpoint(skip_setter, builder_fn = build_return_data)]
    return_data: Option<ReturnData>,

    target_type: PhantomData<T>,
}

impl WorkflowDefinitionBuilder<WorkflowDefinition> {
    pub fn steps(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionSteps> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::Steps),
            target_type: PhantomData,
        }
    }

    pub fn variables(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::Variables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowDefinitionBuilder<WorkflowDefinitionSteps> {
    pub fn variables(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionStepsVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

impl WorkflowDefinitionBuilder<WorkflowDefinitionVariables> {
    pub fn steps(self) -> WorkflowDefinitionBuilder<WorkflowDefinitionStepsVariables> {
        WorkflowDefinitionBuilder {
            core: self.core,
            path: self.path,
            file_system: self.file_system,
            return_data: Some(ReturnData::StepsVariables),
            target_type: PhantomData,
        }
    }
}

fn build_return_data<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &WorkflowDefinitionBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match &builder.return_data {
        Some(ReturnData::Steps) => request_builder.query(&[("returnData", "steps")]),
        Some(ReturnData::StepsVariables) => {
            request_builder.query(&[("returnData", "steps,variables")])
        }
        Some(ReturnData::Variables) => request_builder.query(&[("returnData", "variables")]),
        None => request_builder,
    }
}
